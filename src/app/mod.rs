use iced::Command;

use components::modal::modal;
use popups::PopupMsg;

use crate::app::main_page::MainPageMsg;
use crate::commands;
use crate::{commands::CommandMsg, AppState};

mod collection_tree;
mod main_page;
mod panels;
mod popups;

#[derive(Debug, Clone)]
pub enum AppMsg {
    Command(CommandMsg),
    MainPage(MainPageMsg),
    Popup(PopupMsg),
}

impl AppMsg {
    pub fn update(self, state: &mut AppState) -> Command<AppMsg> {
        let cmd = match self {
            AppMsg::Command(msg) => msg.update(state).map(AppMsg::Command),
            AppMsg::MainPage(msg) => msg.update(state).map(AppMsg::MainPage),
            AppMsg::Popup(msg) => msg.update(state).map(AppMsg::Popup),
        };
        Command::batch([cmd, commands::background(state).map(AppMsg::Command)])
    }
}

pub fn view(state: &AppState) -> iced::Element<AppMsg> {
    let main_page = main_page::view(state).map(AppMsg::MainPage);

    if let Some(ref popup) = state.popup {
        let popup = popups::view(state, popup).map(AppMsg::Popup);
        modal(main_page, popup)
    } else {
        main_page
    }
}
