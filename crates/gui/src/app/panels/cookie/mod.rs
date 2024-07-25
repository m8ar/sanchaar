use crate::state::AppState;
use iced::{
    widget::{text, Column},
    Task,
};

#[derive(Debug, Clone)]
pub enum CookieTabMsg {}

impl CookieTabMsg {
    pub fn update(self) -> Task<Self> {
        Task::none()
    }
}

pub fn view<'a>(state: &AppState) -> iced::Element<'a, CookieTabMsg> {
    "hello".into()
}
