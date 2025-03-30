use axum::{extract, Json};
use chrono::{DateTime, Utc};
use futures::{stream::FuturesUnordered, StreamExt};
use http::HeaderMap;
use rustc_hash::FxHashSet;
use tokio::sync::Mutex;

use crate::{
    api::{
        crud::GetItemsReturn,
        rss::{get_channel_for_source, get_image_from_link},
    },
    db::{Item, Source},
    ApiError,
};

use super::{auth::is_authorized, State};

pub async fn preview_source(
    extract::State(state): extract::State<State>,
    headers: HeaderMap,
    Json(source): Json<Source>,
) -> Result<Json<Vec<GetItemsReturn>>, ApiError> {
    is_authorized(&state.config, &headers).await?;
    let now = Utc::now().naive_utc();
    let channel = get_channel_for_source(&state.client, &source).await?;
    let channel_tags = channel
        .categories
        .into_iter()
        .map(|c| c.name)
        .collect::<Vec<_>>();
    let preview = Mutex::new(Vec::with_capacity(channel.items.len()));

    let mut futures = channel.items.into_iter().map(|channel_item| async {
        let Some(link) = channel_item.link else {
            tracing::error!("item has no link");
            return;
        };
        let pub_date = channel_item
            .pub_date
            .and_then(|s| DateTime::parse_from_rfc2822(&s).ok());
        if let Some(min_date) = source.min_date {
            if let Some(pub_date) = pub_date {
                if pub_date < min_date.and_local_timezone(pub_date.timezone()).unwrap() {
                    // If item is older than the min_date for this source, ignore it
                    tracing::debug!("Ignoring {link} because its too old.");
                    return;
                }
            }
        }

        let Ok(image) = get_image_from_link(&state.client, &link).await else {
            tracing::error!("Error getting image for {link}");
            return;
        };

        let item = Item {
            // Filled in by db
            id: 0,
            created_at: now,
            updated_at: now,

            title: channel_item.title,
            link,
            author: channel_item.author,
            description: channel_item.description,
            favorite: false,
            done: false,
            published: pub_date.as_ref().map(DateTime::naive_local),
            image,
            source_id: Some(source.id),
            source_link: Some(source.url.clone()),
        };
        let mut item_tags = channel_item
            .categories
            .into_iter()
            .map(|c| c.name.to_ascii_lowercase())
            .collect::<FxHashSet<_>>();
        item_tags.extend(channel_tags.iter().cloned());

        preview.lock().await.push(GetItemsReturn {
            item,
            tags: item_tags,
        });
    }).collect::<FuturesUnordered<_>>();

    while let Some(_) = futures.next().await {
        // Poll all tasks to completion
    }
    drop(futures);

    Ok(Json(preview.into_inner()))
}
