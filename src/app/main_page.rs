use iced::font::Weight;
use iced::widget::text::Shaping::Advanced;
use iced::widget::{text, Column};
use iced::Font;
use iced::Length::Shrink;

use crate::components::{card_tab, card_tabs, TabBarAction};
use crate::panels;
use crate::panels::PanelMsg;
use crate::state::request::Method;
use crate::state::{AppState, TabKey};

#[derive(Debug, Clone)]
pub enum MainPageMsg {
    TabBarAction(TabBarAction<TabKey>),
    Panel(PanelMsg),
}

impl MainPageMsg {
    pub fn update(self, state: &mut AppState) {
        match self {
            Self::TabBarAction(action) => match action {
                TabBarAction::ChangeTab(tab) => {
                    state.active_tab = tab;
                }
                TabBarAction::NewTab => {
                    let tab = state.tabs.insert(Default::default());
                    state.active_tab = tab;
                }
                TabBarAction::CloseTab(key) => {
                    state.tabs.remove(key);
                    if state.tabs.is_empty() {
                        let tab = state.tabs.insert(Default::default());
                        state.active_tab = tab;
                    } else if state.active_tab == key {
                        state.active_tab = state.tabs.keys().next().unwrap();
                    }
                }
            },
            Self::Panel(msg) => msg.update(state),
        }
    }
}

fn method_color(method: Method) -> iced::Color {
    match method {
        Method::GET => iced::Color::from_rgb8(0, 0, 255),
        Method::POST => iced::Color::from_rgb8(0, 180, 0),
        Method::PUT => iced::Color::from_rgb8(255, 165, 0),
        Method::DELETE => iced::Color::from_rgb8(200, 0, 0),
        Method::PATCH => iced::Color::from_rgb8(128, 0, 128),
        Method::HEAD => iced::Color::from_rgb8(0, 0, 0),
        Method::OPTIONS => iced::Color::from_rgb8(0, 128, 128),
        Method::CONNECT => iced::Color::from_rgb8(255, 0, 255),
        Method::TRACE => iced::Color::from_rgb8(150, 150, 150),
    }
}

pub fn view(state: &AppState) -> iced::Element<MainPageMsg> {
    let mut tabs = state.tabs.iter().collect::<Vec<_>>();
    tabs.sort_unstable_by_key(|(k, _v)| *k);

    let tabs = tabs
        .into_iter()
        .map(|(k, v)| {
            card_tab(
                k,
                text(v.request.method)
                    .style(iced::theme::Text::Color(method_color(v.request.method)))
                    .shaping(Advanced)
                    .size(12)
                    .height(Shrink)
                    .font(Font {
                        weight: Weight::Semibold,
                        ..Default::default()
                    }),
                text("Untitled"),
            )
        })
        .collect();

    Column::new()
        .push(card_tabs(
            state.active_tab,
            tabs,
            MainPageMsg::TabBarAction,
            None,
        ))
        .push(panels::view(state).map(MainPageMsg::Panel))
        .spacing(8)
        .padding(4)
        .into()
}
