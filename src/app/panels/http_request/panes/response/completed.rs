use humansize::{format_size, BINARY};
use iced::widget::{button, container, text, Column, Row};
use iced::{clipboard, Alignment, Border, Color, Task, Element, Theme};

use components::{
    button_tab, button_tabs, code_editor, key_value_viewer, CodeEditorMsg, ContentType,
};

use crate::state::response::{BodyMode, CompletedResponse, ResponseState};
use crate::state::{response::ResponseTabId, AppState};

#[derive(Debug, Clone)]
pub enum CompletedMsg {
    TabChanged(ResponseTabId),
    CodeViewerMsg(CodeEditorMsg),
    SetBodyMode(BodyMode),
    CopyBodyToClipboard,
}

impl CompletedMsg {
    pub(crate) fn update(self, state: &mut AppState) -> Task<CompletedMsg> {
        let active_tab = state.active_tab_mut();
        match self {
            Self::TabChanged(tab) => {
                active_tab.response.active_tab = tab;
            }
            Self::CodeViewerMsg(msg) => {
                if let ResponseState::Completed(ref mut res) = active_tab.response.state {
                    msg.update(res.selected_content_mut());
                }
            }
            Self::SetBodyMode(mode) => {
                if let ResponseState::Completed(ref mut res) = active_tab.response.state {
                    res.mode = mode;
                }
            }
            CompletedMsg::CopyBodyToClipboard => {
                if let ResponseState::Completed(ref res) = active_tab.response.state {
                    return clipboard::write(res.selected_content().text());
                }
            }
        }
        Task::none()
    }
}

fn fmt_duration(d: std::time::Duration) -> String {
    let millis = d.as_millis();

    let mut duration = String::from("Time:");
    if millis > 1000 {
        duration.push_str(&format!(" {}s", millis / 1000));
    }
    let millis = millis % 1000;
    if millis > 0 {
        duration.push_str(&format!(" {}ms", millis));
    }

    duration
}

fn status_color(status: reqwest::StatusCode) -> Color {
    match status.as_u16() {
        200..=299 => Color::from_rgb8(0, 200, 0),
        300..=399 => Color::from_rgb8(0, 0, 200),
        400..=499 => Color::from_rgb8(200, 200, 0),
        500..=599 => Color::from_rgb8(200, 0, 0),
        _ => Color::WHITE,
    }
}

fn body_view(cr: &CompletedResponse) -> Element<CompletedMsg> {
    fn button_style(theme: &Theme, _status: button::Status, selected: bool) -> button::Style {
        if selected {
            button::secondary(theme, button::Status::Active)
        } else {
            button::text(theme, button::Status::Active)
        }
    }

    let mode = cr.mode;
    let size = 14;
    let actions = Row::new()
        .push(
            button(text("Preview").size(size))
                .padding([2, 4])
                .on_press(CompletedMsg::SetBodyMode(BodyMode::Pretty))
                .style(move |t, s| button_style(t, s, BodyMode::Pretty == mode)),
        )
        .push(
            button(text("Raw").size(size))
                .padding([2, 4])
                .on_press(CompletedMsg::SetBodyMode(BodyMode::Raw))
                .style(move |t, s| button_style(t, s, BodyMode::Raw == mode)),
        )
        .spacing(2);

    let action_bar = Row::new()
        .push(container(actions).style(|theme: &Theme| {
            container::Style {
                border: Border::default()
                    .with_width(1)
                    .with_color(theme.extended_palette().background.weak.color),
                ..container::transparent(theme)
            }
        }))
        .push(
            button(text("Copy").size(size))
                .padding([2, 4])
                .style(button::secondary)
                .on_press(CompletedMsg::CopyBodyToClipboard),
        )
        .spacing(8);

    let content = cr.selected_content();
    Column::new()
        .push(action_bar)
        .push(code_editor(content, ContentType::Json).on_action(CompletedMsg::CodeViewerMsg))
        .spacing(4)
        .height(iced::Length::Fill)
        .width(iced::Length::Fill)
        .into()
}

pub(crate) fn view<'a>(
    state: &'a AppState,
    cr: &'a CompletedResponse,
) -> Element<'a, CompletedMsg> {
    let active_tab = state.active_tab();
    let res = &cr.result;

    let status_size = 12;
    let status = Row::new()
        .push(
            text(res.status.to_string())
                .size(status_size)
                .color(status_color(res.status)),
        )
        .push(
            text(format_size(res.size_bytes, BINARY))
                .size(status_size)
                .color(Color::from_rgb8(182, 128, 182)),
        )
        .push(
            text(fmt_duration(res.duration))
                .size(status_size)
                .color(Color::from_rgb8(160, 160, 160)),
        )
        .padding([4, 8])
        .spacing(8)
        .align_items(Alignment::Center);

    let headers = res
        .headers
        .iter()
        .map(|(k, v)| (k.as_str(), v.to_str().unwrap_or_default()))
        .collect::<Vec<_>>();

    let tab_content = match active_tab.response.active_tab {
        ResponseTabId::Body => body_view(cr),
        ResponseTabId::Headers => key_value_viewer(&headers),
    };

    let tabs = button_tabs(
        active_tab.response.active_tab,
        [
            button_tab(ResponseTabId::Body, || text("Body")),
            button_tab(ResponseTabId::Headers, || text("Headers")),
        ]
        .into_iter(),
        CompletedMsg::TabChanged,
        Some(status.into()),
    );

    Column::new()
        .push(tabs)
        .push(tab_content)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .spacing(4)
        .into()
}
