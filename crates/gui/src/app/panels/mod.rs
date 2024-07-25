use iced::widget::container;
use iced::Task;

use crate::state::{AppState, Tab};

pub mod collection;
pub mod cookie;
pub mod http;

#[derive(Debug, Clone)]
pub enum PanelMsg {
    HttpTab(http::HttpTabMsg),
    CollectionTab(collection::CollectionTabMsg),
    CookieTab(cookie::CookieTabMsg),
}

impl PanelMsg {
    pub fn update(self, state: &mut AppState) -> Task<Self> {
        match self {
            PanelMsg::HttpTab(msg) => msg.update(state).map(PanelMsg::HttpTab),
            PanelMsg::CollectionTab(msg) => msg.update(state).map(PanelMsg::CollectionTab),
            PanelMsg::CookieTab(msg) => msg.update().map(PanelMsg::CookieTab),
        }
    }
}

pub fn view<'a>(state: &'a AppState, tab: &'a Tab) -> iced::Element<'a, PanelMsg> {
    let req = match tab {
        Tab::Http(tab) => http::view(state, tab).map(PanelMsg::HttpTab),
        Tab::Collection(tab) => collection::view(tab).map(PanelMsg::CollectionTab),
        Tab::Cookie(tab) => cookie::view(state).map(PanelMsg::CookieTab),
    };

    container::Container::new(req)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .into()
}
