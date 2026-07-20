#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod data;
mod http;
mod models;
mod queries;

use std::time::Duration;

use freya::{
    prelude::*,
    query::{Query, use_query},
    router::*,
};

use crate::{models::GetItemsReturn, queries::GetItems};

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    Router::<Route>::new(|| RouterConfig::default().with_initial_path(Route::Home))
}

#[derive(PartialEq)]
struct Layout;
impl Component for Layout {
    fn render(&self) -> impl IntoElement {
        rect().center().expanded().child(Outlet::<Route>::new())
    }
}

#[derive(PartialEq)]
struct Home {}
impl Component for Home {
    fn render(&self) -> impl IntoElement {
        let res = use_query(Query::new((), GetItems).interval_time(Duration::from_mins(1)));

        let content = match &*res.read().state() {
            freya::query::QueryStateData::Pending => "ready!".into_element(),
            freya::query::QueryStateData::Loading { res } => "loading...".into_element(),
            freya::query::QueryStateData::Settled {
                res,
                settlement_instant,
            } => rect()
                .direction(Direction::Vertical)
                .main_align(Alignment::Center)
                .padding(8.)
                .spacing(8.)
                .children(
                    res.as_ref()
                        .unwrap()
                        .iter()
                        .map(|item| Card { item: item.clone() }.into_element()),
                )
                .into_element(),
        };

        ScrollView::new().direction(Direction::Vertical).child(content)
    }
}

#[derive(Debug, PartialEq)]
struct Card {
    item: GetItemsReturn,
}

impl Component for Card {
    fn render(&self) -> impl IntoElement {
        Link::new(&*self.item.item.link).child(
            self.item
                .item
                .title
                .as_deref()
                .unwrap_or(&self.item.item.link),
        )
    }

    fn render_key(&self) -> DiffKey {
        DiffKey::U64(self.item.item.id as u64)
    }
}

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
        #[route("/")]
        Home,
}
