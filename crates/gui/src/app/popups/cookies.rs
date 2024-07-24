use std::borrow::Cow;

use components::{button_tab, button_tabs};
use iced::widget::{horizontal_space, pick_list, text, Column, Row};
use iced::{Element, Task, Theme};

use crate::state::popups::{AppSettingTabs, AppSettingsState, Popup};
use crate::state::AppState;

#[derive(Debug, Clone)]
pub enum Message {
    Done,
}

impl Message {
    pub fn update(self, state: &mut AppState) -> Task<Message> {
        let Some(Popup::AppSettings(data)) = state.popup.as_mut() else {
            return Task::none();
        };

        match self {
            Message::Done => {
                state.popup = None;
            }
        }
        Task::none()
    }
}

pub fn title<'a>() -> Cow<'a, str> {
    Cow::Borrowed("Settings")
}

pub fn done() -> Option<Message> {
    Some(Message::Done)
}

pub(crate) fn view<'a>(state: &'a AppState) -> Element<Message> {
    text("Settings").into()
}
