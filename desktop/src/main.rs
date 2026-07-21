#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod data;
mod http;
mod models;
mod queries;

use std::{rc::Rc, time::Duration};

use freya::{
    prelude::*,
    query::{Query, use_query},
    router::*,
    webview::{WebView, WebViewPlugin},
};

use crate::{models::GetItemsReturn, queries::GetItems};

fn main() {
    launch(
        LaunchConfig::new()
            .with_plugin(WebViewPlugin::new())
            .with_window(WindowConfig::new(app)),
    )
}

fn app() -> impl IntoElement {
    let mut theme = use_init_theme(|| Platform::get().preferred_theme.read().to_theme());

    use_side_effect(move || theme.set(Platform::get().preferred_theme.read().to_theme()));

    rect().theme_background().child(Router::<Route>::new(|| {
        RouterConfig::default().with_initial_path(Route::Home)
    }))
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
        let mut post_url = use_state(|| None::<Rc<str>>);
        let res = use_query(Query::new((), GetItems).interval_time(Duration::from_mins(1)));

        if let Some(post_url_str) = post_url.read().as_deref() {
            return rect()
                .direction(Direction::Vertical)
                .child(
                    Button::new()
                        .filled()
                        .padding(8.)
                        .cursor_icon(CursorIcon::Pointer)
                        .on_press(move |_| *post_url.write() = None)
                        .child("Back"),
                )
                .child(WebView::new(post_url_str).expanded())
                .into_element();
        }

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
                .children(res.as_ref().unwrap().iter().map(|item| {
                    Card {
                        item: item.clone(),
                        post_url: post_url.clone(),
                    }
                    .into_element()
                }))
                .into_element(),
        };

        ScrollView::new()
            .direction(Direction::Vertical)
            .child(content)
            .into_element()
    }
}

#[derive(PartialEq)]
struct Card {
    item: GetItemsReturn,
    post_url: State<Option<Rc<str>>>,
}

impl Component for Card {
    fn render(&self) -> impl IntoElement {
        let mut post_url = self.post_url.clone();
        let link: Rc<str> = Rc::from(&*self.item.item.link);

        freya::components::Card::new()
            .child(
                label()
                    .theme_color()
                    .font_weight(FontWeight::BOLD)
                    .font_size(24.)
                    .on_press(move |_| {
                        Cursor::set(CursorIcon::default());
                        *post_url.write() = Some(link.clone());
                    })
                    .on_pointer_enter(move |_| {
                        Cursor::set(CursorIcon::Pointer);
                    })
                    .on_pointer_leave(move |_| {
                        Cursor::set(CursorIcon::default());
                    })
                    .text(
                        self.item
                            .item
                            .title
                            .clone()
                            .unwrap_or(self.item.item.link.clone()),
                    ),
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
