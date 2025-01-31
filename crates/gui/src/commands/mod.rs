use core::http::collection::Collection;
use core::persistence::collections;
use iced::Task;
use std::time::Instant;

use crate::{
    app::AppMsg,
    state::{collection_tab::CollectionTab, RequestDirtyState, Tab, TabKey},
    AppState,
};

use self::builders::{check_dirty_requests_cmd, load_collections_cmd};

pub mod builders;
mod cancellable_task;
pub mod dialog;

#[derive(Debug, Clone)]
pub struct JobState {
    task: BackgroundTask,
    done: bool,
    started: Instant,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum BackgroundTask {
    SaveCollections,
    CheckDirtyRequests,
}

fn remove_task(state: &mut AppState, task: BackgroundTask) {
    state.background_tasks.retain(|t| t.task != task);
}

fn task_done(state: &mut AppState, task: BackgroundTask) {
    if let Some(job) = state.background_tasks.iter_mut().find(|t| t.task == task) {
        job.done = true;
    }
}

fn schedule_task(state: &mut AppState, task: BackgroundTask, delay: u64) -> bool {
    let job = state.background_tasks.iter().find(|t| t.task == task);

    let sch = match job {
        Some(job) => job.started.elapsed().as_secs() > delay && job.done,
        None => true,
    };
    if sch {
        remove_task(state, task);
        state.background_tasks.push(JobState {
            task,
            done: false,
            started: Instant::now(),
        });
    }
    sch
}

#[derive(Debug, Clone)]
pub enum TaskMsg {
    CollectionsLoaded(Vec<Collection>),
    Completed(BackgroundTask),
    UpdateDirtyTabs(Vec<(TabKey, RequestDirtyState)>),
}

impl TaskMsg {
    pub fn update(self, state: &mut AppState) -> Task<Self> {
        match self {
            TaskMsg::CollectionsLoaded(collection) => {
                state.collections.insert_all(collection);
                let last = state.collections.iter().last();
                if let Some((key, col)) = last {
                    state.open_tab(Tab::Collection(CollectionTab::new(key, col)));
                }
                task_done(state, BackgroundTask::SaveCollections);
            }
            TaskMsg::Completed(task) => {
                task_done(state, task);
            }
            TaskMsg::UpdateDirtyTabs(status) => {
                task_done(state, BackgroundTask::CheckDirtyRequests);
                for (key, status) in status {
                    if let Some(Tab::Http(tab)) = state.tabs.get_mut(&key) {
                        tab.request_dirty_state = status;
                    };
                }
            }
        };
        Task::none()
    }
}

fn save_open_collections(state: &mut AppState) -> Task<TaskMsg> {
    let task = BackgroundTask::SaveCollections;
    let schedule = state.collections.dirty && schedule_task(state, task, 0);
    if !schedule {
        return Task::none();
    }

    let collections = state.collections.get_collections_for_save();
    Task::perform(collections::save(collections), |result| match result {
        Ok(_) => TaskMsg::Completed(BackgroundTask::SaveCollections),
        Err(e) => {
            log::error!("Error saving collections: {:?}", e);
            TaskMsg::Completed(BackgroundTask::SaveCollections)
        }
    })
}

fn check_dirty_requests(state: &mut AppState) -> Task<TaskMsg> {
    let task = BackgroundTask::CheckDirtyRequests;
    if !schedule_task(state, task, 2) {
        return Task::none();
    }

    check_dirty_requests_cmd(state, TaskMsg::UpdateDirtyTabs)
}

pub fn background(state: &mut AppState) -> Task<TaskMsg> {
    Task::batch([save_open_collections(state), check_dirty_requests(state)])
}

pub fn init_command() -> Task<AppMsg> {
    Task::perform(load_collections_cmd(), TaskMsg::CollectionsLoaded).map(AppMsg::Command)
}
