use std::time::Duration;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::Utc;
use http::HeaderMap;
use serde::{Deserialize, Serialize};

use crate::{
    db::{Item, Source, Tag},
    ApiError,
};

use super::{auth::is_authorized, rss::get_channel_for_source};

pub async fn create_tag(
    State(state): State<super::State>,
    headers: HeaderMap,
    Json(mut tag): Json<Tag>,
) -> Result<Json<Tag>, ApiError> {
    is_authorized(&state.config, &headers).await?;
    tag.insert(&state.sqlite).await?;
    Ok(Json(tag))
}

pub async fn delete_tag(
    State(state): State<super::State>,
    headers: HeaderMap,
    Path(name): Path<String>,
) -> Result<(), ApiError> {
    is_authorized(&state.config, &headers).await?;
    Tag::delete(&name, &state.sqlite).await?;
    Ok(())
}

pub async fn get_tag(
    State(state): State<super::State>,
    Path(name): Path<String>,
) -> Result<Json<Tag>, ApiError> {
    Ok(Json(
        Tag::get_by_name(&name, &state.sqlite)
            .await?
            .ok_or(ApiError::NotFound)?,
    ))
}

pub async fn update_tag(
    State(state): State<super::State>,
    headers: HeaderMap,
    Json(mut tag): Json<Tag>,
) -> Result<(), ApiError> {
    is_authorized(&state.config, &headers).await?;
    tag.update(&state.sqlite).await?;
    Ok(())
}

pub async fn get_tags(State(state): State<super::State>) -> Result<Json<Vec<Tag>>, ApiError> {
    Ok(Json(Tag::get_all(&state.sqlite).await?))
}

pub async fn create_item(
    State(state): State<super::State>,
    headers: HeaderMap,
    Json(mut item): Json<Item>,
) -> Result<Json<Item>, ApiError> {
    is_authorized(&state.config, &headers).await?;
    item.insert(&state.sqlite).await?;
    Ok(Json(item))
}

pub async fn delete_item(
    State(state): State<super::State>,
    headers: HeaderMap,
    Path(id): Path<i64>,
) -> Result<(), ApiError> {
    is_authorized(&state.config, &headers).await?;
    Item::delete(id, &state.sqlite).await?;
    Ok(())
}

pub async fn get_item(
    State(state): State<super::State>,
    Path(id): Path<i64>,
) -> Result<Json<Item>, ApiError> {
    Ok(Json(
        Item::get_by_id(id, &state.sqlite)
            .await?
            .ok_or(ApiError::NotFound)?,
    ))
}

#[derive(Debug, Deserialize)]
pub struct GetItemsQuery {
    #[serde(with = "humantime_serde")]
    from_last: Duration,
    #[serde(default)]
    include_done: bool,
}

#[derive(Debug, Serialize)]
pub struct GetItemsReturn {
    #[serde(flatten)]
    item: Item,
    tags: Vec<String>,
}

pub async fn get_items(
    State(state): State<super::State>,
    Query(query): Query<GetItemsQuery>,
) -> Result<Json<Vec<GetItemsReturn>>, ApiError> {
    tracing::debug!("{:?}", query.from_last);
    Ok(Json(
        Item::feed(query.from_last, query.include_done, &state.sqlite)
            .await?
            .into_iter()
            .map(|item_w_tags| GetItemsReturn {
                item: item_w_tags.item,
                tags: item_w_tags
                    .tags
                    .unwrap_or_default()
                    .split(",")
                    .map(ToString::to_string)
                    .collect(),
            })
            .collect(),
    ))
}

pub async fn done(
    State(state): State<super::State>,
    Path(id): Path<i64>,
    headers: HeaderMap,
) -> Result<(), ApiError> {
    is_authorized(&state.config, &headers).await?;
    Item::set_done(id, true, &state.sqlite).await?;
    Ok(())
}

pub async fn create_source(
    State(state): State<super::State>,
    headers: HeaderMap,
    Json(mut source): Json<Source>,
) -> Result<Json<Source>, ApiError> {
    is_authorized(&state.config, &headers).await?;

    // Check that the channel actual exists and populate last_pub and ttl
    let channel = get_channel_for_source(&state.client, &source).await?;
    source.last_pub = channel
        .pub_date
        .and_then(|pub_date| {
            chrono::DateTime::parse_from_rfc2822(&pub_date)
                .ok()
                .map(|dt| dt.naive_utc())
        })
        .unwrap_or(Utc::now().naive_utc());
    source.ttl = channel.ttl.and_then(|ttl| ttl.parse().ok());

    source.insert(&state.sqlite).await?;
    state.poll_send.send(()).await.ok();
    Ok(Json(source))
}

pub async fn delete_source(
    State(state): State<super::State>,
    headers: HeaderMap,
    Path(id): Path<i64>,
) -> Result<(), ApiError> {
    is_authorized(&state.config, &headers).await?;
    Source::delete(id, &state.sqlite).await?;
    Ok(())
}

pub async fn get_source(
    State(state): State<super::State>,
    Path(id): Path<i64>,
) -> Result<Json<Source>, ApiError> {
    Ok(Json(
        Source::get_by_id(id, &state.sqlite)
            .await?
            .ok_or(ApiError::NotFound)?,
    ))
}

pub async fn get_sources(State(state): State<super::State>) -> Result<Json<Vec<Source>>, ApiError> {
    Ok(Json(Source::get_all(&state.sqlite).await?))
}
