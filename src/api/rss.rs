use std::{
    error::Error,
    ops::{Deref, DerefMut},
    sync::{Arc, LazyLock},
    time::Duration,
};

use chrono::DateTime;
use futures::{stream::FuturesUnordered, StreamExt};
use http::Uri;
use rustc_hash::FxHashSet;
use sqlx::{Pool, Sqlite};
use tokio::{
    select,
    sync::{broadcast, mpsc},
};
use ts_rs::TS;

use crate::{
    config::Config,
    continue_on_err,
    db::{Item, Source, Tag},
    ApiError,
};

const CHECK_INTERVAL: Duration = Duration::from_secs(60);

#[derive(Debug, Clone, Copy, PartialEq, Eq, TS)]
#[ts(export, export_to = "../web/src/types/PollMessage.ts")]
pub enum PollMessage {
    Polling,
    PollDone,
}

#[derive(Debug)]
pub struct CloneReceiver<T>(broadcast::Receiver<T>);

pub fn start_poller(
    _config: Arc<Config>,
    client: reqwest::Client,
    sqlite: Pool<Sqlite>,
) -> (CloneReceiver<PollMessage>, mpsc::Sender<()>) {
    let (msg_send, msg_recv) = broadcast::channel(128);
    let (poll_send, mut poll_recv) = mpsc::channel::<()>(1);

    tokio::spawn(async move {
        loop {
            msg_send.send(PollMessage::Polling).unwrap();
            let now = chrono::Utc::now().naive_utc();
            let sources = continue_on_err!(Source::get_all(&sqlite).await);
            let client = client.clone();

            // TODO: consider parallelization (i don't really need it personally though)
            for mut source in sources {
                let should_poll = match source.last_poll.as_ref() {
                    Some(last_poll) => {
                        // If last poll was more than ttl (or 60) minutes ago
                        (now - *last_poll).num_minutes() >= source.ttl.unwrap_or(60)
                    }
                    // We've never polled this source (successfully)
                    None => true,
                };

                if !should_poll {
                    tracing::trace!("Skipping poll for {}", source.name);
                    continue;
                }

                tracing::debug!("Polling {}", source.name);
                let channel = continue_on_err!(get_channel_for_source(&client, &source).await);
                let source_tags = continue_on_err!(Source::tags(source.id, &sqlite).await);

                let categories = std::sync::Mutex::new(FxHashSet::with_capacity_and_hasher(
                    channel.items.len() * 3,
                    Default::default(),
                ));
                let items = std::sync::Mutex::new(Vec::with_capacity(channel.items.len()));

                let mut futures = channel
                    .items
                    .into_iter()
                    .map(|channel_item| {
                        async {
                            let link = channel_item.link.ok_or_else(|| "item has no link")?;
                            let pub_date = channel_item
                                .pub_date
                                .and_then(|s| DateTime::parse_from_rfc2822(&s).ok());
                            if let Some(min_date) = source.min_date {
                                if let Some(pub_date) = pub_date {
                                    if pub_date
                                        < min_date.and_local_timezone(pub_date.timezone()).unwrap()
                                    {
                                        // If item is older than the min_date for this source, ignore it
                                        tracing::debug!("Ignoring {link} because its too old.");
                                        return Ok(());
                                    }
                                }
                            }

                            let image = get_image_from_link(&client, &link).await?;

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
                            let item_categories = channel_item
                                .categories
                                .into_iter()
                                .filter(|c| !c.name.is_empty())
                                .map(|c| Arc::<str>::from(c.name.to_ascii_lowercase().as_str()))
                                .collect::<FxHashSet<_>>();

                            let mut categories = categories.lock().unwrap();
                            for cat in &item_categories {
                                categories.insert(cat.clone());
                            }
                            items.lock().unwrap().push((item, item_categories));
                            Ok::<(), Box<dyn Error + 'static>>(())
                        }
                    })
                    .collect::<FuturesUnordered<_>>();

                while let Some(item_result) = futures.next().await {
                    if let Err(err) = item_result {
                        tracing::error!("Error while creating item from {}: {err:?}", source.name);
                    }
                }
                drop(futures);

                // We can consider the polling done at this point and update the row
                source.last_pub = channel
                    .pub_date
                    .and_then(|pub_date| {
                        chrono::DateTime::parse_from_rfc2822(&pub_date)
                            .ok()
                            .map(|dt| dt.naive_utc())
                    })
                    .unwrap_or(now);
                source.last_poll = Some(now);
                source.ttl = channel
                    .ttl
                    .as_deref()
                    .and_then(|ttl_str| ttl_str.parse().ok());
                if let Err(err) = source.update(&sqlite).await {
                    tracing::error!("Error updating row for {}: {err:?}", source.name);
                };
                msg_send.send(PollMessage::PollDone).ok();

                // Try to create tags for each category we found in the items
                let category_tags = categories
                    .into_inner()
                    .unwrap()
                    .iter()
                    .map(|category| Tag {
                        created_at: now,
                        updated_at: now,
                        text_color: None,
                        background_color: None,
                        border_color: None,

                        name: category.to_string(),
                    })
                    .collect::<Vec<_>>();
                if let Err(err) = Tag::insert_many(&category_tags, &sqlite).await {
                    tracing::error!("Failed to create tags from categories: {err:?}");
                };

                for (mut item, mut item_categories) in items.into_inner().unwrap() {
                    match item.insert(&sqlite).await {
                        Ok(_) => {
                            tracing::info!("Inserted new item for {}", item.link);
                            // Add tags from the source
                            for source_tag in &source_tags {
                                item_categories.insert(Arc::from(source_tag.name.as_str()));
                            }

                            // Now add the tags to the item
                            if let Err(err) = Item::add_tags(
                                item.id,
                                &item_categories.iter().map(Deref::deref).collect::<Vec<_>>(),
                                &sqlite,
                            )
                            .await
                            {
                                tracing::error!("Failed to add tags to {}: {err:?}", item.link);
                            };
                        }
                        Err(err) => match err.into_sqlx_error() {
                            sqlx::Error::Database(db_err)
                                if db_err.kind() == sqlx::error::ErrorKind::UniqueViolation =>
                            {
                                tracing::debug!(
                                    "Tried to insert link {} that already exists",
                                    item.link
                                );
                            }
                            err => {
                                tracing::error!("Error while adding item: {err:?}");
                            }
                        },
                    }
                }
            }

            // Wait for interval or be notified of immediate poll
            select! {
                _ = tokio::time::sleep(CHECK_INTERVAL) => {},
                _ = poll_recv.recv() => {}
            };
        }
    });

    (CloneReceiver(msg_recv), poll_send)
}

pub async fn get_channel_for_source(
    client: &reqwest::Client,
    source: &Source,
) -> Result<rss::Channel, ApiError> {
    let res = client.get(&source.url).send().await?;
    Ok(rss::Channel::read_from(&*res.bytes().await?)?)
}

pub async fn get_image_from_link(
    client: &reqwest::Client,
    link: &str,
) -> Result<Option<String>, Box<dyn Error + 'static>> {
    static SELECTOR: LazyLock<scraper::Selector> =
        LazyLock::new(|| scraper::Selector::parse("head > meta[property=\"og:image\"]").unwrap());

    let res = client.get(link).send().await?;
    let page = scraper::Html::parse_document(&res.text().await?);
    if !page.errors.is_empty() {
        tracing::error!("Html parse errors for {link}: {:?}", page.errors);
        return Ok(None);
    }

    let elements = page.select(&SELECTOR);
    let candidates = elements
        .filter_map(|el| {
            el.attr("content")
                .and_then(|content| content.parse::<Uri>().ok())
                .filter(|content| {
                    let path = content.path();
                    path.ends_with("jpg")
                        || path.ends_with("jpeg")
                        || path.ends_with("png")
                        || path.ends_with("webp")
                })
        })
        .collect::<Vec<_>>();

    tracing::debug!("Found thumbnail candidates for {link}: {candidates:?}");

    // TODO: maybe do more validation on images?? (idc)
    Ok(candidates.get(0).map(ToString::to_string))
}

impl<T> Deref for CloneReceiver<T> {
    type Target = broadcast::Receiver<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for CloneReceiver<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Clone> Clone for CloneReceiver<T> {
    fn clone(&self) -> Self {
        Self(self.0.resubscribe())
    }
}

#[macro_export]
macro_rules! continue_on_err {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => {
                tracing::error!("Error while polling sources: {err:?}");
                continue;
            }
        }
    };
}
