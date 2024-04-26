use iced::alignment::Horizontal;
use iced::widget::container::Style;
use iced::widget::{button, container, horizontal_space, text, Column, Row};
use iced::{Border, Command, Element};

use crate::state::popups::Popup;
use crate::state::AppState;

mod create_collection;
mod environment_editor;

#[derive(Clone, Debug)]
pub enum PopupMsg {
    CreateCollection(create_collection::Message),
    EnvironmentEditor(environment_editor::Message),
    ClosePopup,
}

impl PopupMsg {
    pub fn update(self, state: &mut AppState) -> Command<PopupMsg> {
        match self {
            Self::CreateCollection(msg) => msg.update(state).map(PopupMsg::CreateCollection),
            Self::EnvironmentEditor(msg) => msg.update(state).map(PopupMsg::EnvironmentEditor),
            Self::ClosePopup => {
                state.popup = None;
                Command::none()
            }
        }
    }
}

pub fn view<'a>(state: &'a AppState, popup: &'a Popup) -> Element<'a, PopupMsg> {
    let (title, content, done_msg) = match popup {
        Popup::CreateCollection(ref data) => (
            create_collection::title(),
            create_collection::view(state, data).map(PopupMsg::CreateCollection),
            create_collection::done(data).map(PopupMsg::CreateCollection),
        ),
        Popup::EnvironmentEditor(tab) => (
            environment_editor::title(),
            environment_editor::view(state, *tab).map(PopupMsg::EnvironmentEditor),
            environment_editor::done().map(PopupMsg::EnvironmentEditor),
        ),
    };

    let buttons = Row::new()
        .push(horizontal_space())
        .push(
            button("Cancel")
                .style(button::secondary)
                .on_press(PopupMsg::ClosePopup),
        )
        .push(
            button("Done")
                .style(button::primary)
                .on_press_maybe(done_msg),
        )
        .width(iced::Length::Shrink)
        .spacing(8);

    container(
        Column::new()
            .push(text(title).size(20))
            .push(content)
            .push(buttons)
            .width(iced::Length::Shrink)
            .height(iced::Length::Shrink)
            .spacing(12),
    )
    .padding(16)
    .style(|theme| Style {
        background: Some(theme.extended_palette().background.weak.color.into()),
        border: Border::rounded(6),
        ..Style::default()
    })
    .into()
}
