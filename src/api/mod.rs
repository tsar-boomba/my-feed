mod auth;
mod crud;
mod rss;

use std::{sync::Arc, time::Duration};

use auth::login;
use axum::{
    routing::{get, post},
    Router,
};
use crud::{
    create_item, create_source, create_tag, delete_item, delete_source, delete_tag, done, get_item, get_items, get_source, get_sources, get_tag, get_tags
};
use rss::{CloneReceiver, PollMessage};
use sqlx::{Pool, Sqlite};
use tokio::sync::mpsc;

use crate::config::Config;

#[derive(Debug, Clone)]
struct State {
    config: Arc<Config>,
    sqlite: Pool<Sqlite>,
    poll_recv: CloneReceiver<PollMessage>,
    poll_send: mpsc::Sender<()>,
    client: reqwest::Client,
}

pub fn api_router(config: Arc<Config>, sqlite: Pool<Sqlite>) -> color_eyre::Result<Router> {
    let client = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(5))
        .build()?;
    let (poll_recv, poll_send) = rss::start_poller(config.clone(), client.clone(), sqlite.clone());
    let state = State {
        config,
        sqlite,
        poll_recv,
        poll_send,
        client,
    };
    let router = Router::new()
        .route("/tags", get(get_tags).post(create_tag).delete(delete_tag))
        .route("/tags/{name}", get(get_tag))
        .route(
            "/items",
            get(get_items).post(create_item).delete(delete_item),
        )
        .route("/items/{id}", get(get_item))
        .route("/items/{id}/done", post(done))
        .route(
            "/sources",
            get(get_sources).post(create_source).delete(delete_source),
        )
        .route("/sources/{id}", get(get_source))
        .route("/login", post(login))
        .with_state(state);

    Ok(router)
}
