use iced::widget::container::Style;
use iced::widget::{button, container, horizontal_space, text, Column, Row};
use iced::{border, Alignment, Element, Task};

use crate::state::popups::Popup;
use crate::state::AppState;

mod app_settings;
mod cookies;
mod create_collection;
mod environment_editor;
mod name_popup;
mod save_request;

#[derive(Clone, Debug)]
pub enum PopupMsg {
    CreateCollection(create_collection::Message),
    EnvironmentEditor(environment_editor::Message),
    SaveRequest(save_request::Message),
    RenamePopup(name_popup::Message),
    AppSettings(app_settings::Message),
    ClosePopup,
    Ignore,
    Cookies(cookies::Message),
}

impl PopupMsg {
    pub fn update(self, state: &mut AppState) -> Task<PopupMsg> {
        match self {
            PopupMsg::CreateCollection(msg) => msg.update(state).map(PopupMsg::CreateCollection),
            PopupMsg::EnvironmentEditor(msg) => msg.update(state).map(PopupMsg::EnvironmentEditor),
            PopupMsg::SaveRequest(msg) => msg.update(state).map(PopupMsg::SaveRequest),
            PopupMsg::RenamePopup(msg) => msg.update(state).map(PopupMsg::RenamePopup),
            PopupMsg::AppSettings(msg) => msg.update(state).map(PopupMsg::AppSettings),
            PopupMsg::ClosePopup => {
                Popup::close(state);
                Task::none()
            }
            PopupMsg::Ignore => Task::none(),
            PopupMsg::Cookies(msg) => msg.update(state).map(PopupMsg::Cookies),
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
        Popup::EnvironmentEditor(data) => (
            environment_editor::title(),
            environment_editor::view(state, data).map(PopupMsg::EnvironmentEditor),
            environment_editor::done(data).map(PopupMsg::EnvironmentEditor),
        ),
        Popup::SaveRequest(data) => (
            save_request::title(),
            save_request::view(state, data).map(PopupMsg::SaveRequest),
            save_request::done(data).map(PopupMsg::SaveRequest),
        ),
        Popup::PopupName(data) => (
            name_popup::title(),
            name_popup::view(state, data).map(PopupMsg::RenamePopup),
            name_popup::done(data).map(PopupMsg::RenamePopup),
        ),
        Popup::AppSettings(data) => (
            app_settings::title(),
            app_settings::view(state, data).map(PopupMsg::AppSettings),
            app_settings::done(data).map(PopupMsg::AppSettings),
        ),
        Popup::Cookies() => (
            cookies::title(),
            cookies::view(state).map(PopupMsg::Cookies),
            cookies::done().map(PopupMsg::Cookies),
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
        .width(iced::Length::Fill)
        .align_y(Alignment::End)
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
        border: border::rounded(6),
        ..Style::default()
    })
    .into()
}
