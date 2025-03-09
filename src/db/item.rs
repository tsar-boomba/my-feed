use std::time::Duration;

use futures::TryFutureExt;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::prelude::*;

use super::Error;

#[derive(Debug, FromRow, Serialize, Deserialize, ts_rs::TS)]
#[ts(export, export_to = "../web/src/types/Item.ts")]
pub struct Item {
    #[ts(type = "number")]
    pub id: i64,

    pub link: String,

    pub title: Option<String>,

    pub description: Option<String>,

    pub author: Option<String>,

    pub published: Option<chrono::NaiveDateTime>,

    pub source_link: Option<String>,

    pub image: Option<String>,

    #[serde(default)]
    pub favorite: bool,

    #[serde(default)]
    pub done: bool,

    #[serde(skip_deserializing)]
    pub created_at: chrono::NaiveDateTime,

    #[serde(skip_deserializing)]
    pub updated_at: chrono::NaiveDateTime,

    pub source_id: Option<i64>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct ItemWTags {
    #[serde(flatten)]
    #[sqlx(flatten)]
    pub item: Item,
    pub tags: Option<String>,
}

impl Item {
    pub async fn get_all(
        executor: impl Executor<'_, Database = super::DB>,
    ) -> Result<Vec<Self>, Error> {
        sqlx::query_as!(Item, "SELECT * FROM items")
            .fetch_all(executor)
            .await
            .map_err(|e| Error::SelectError("items", e))
    }

    pub async fn feed(
        duration: Duration,
        include_done: bool,
        executor: impl Executor<'_, Database = super::DB>,
    ) -> Result<Vec<ItemWTags>, Error> {
        let cutoff_date_time = (chrono::Utc::now() - duration).naive_utc();
        sqlx::query_as(
            r#"
            SELECT i.*, GROUP_CONCAT(t.name, ',') AS tags
            FROM items i
            LEFT JOIN items_to_tags it ON i.id = it.item_id
            LEFT JOIN tags t ON it.tag_id = t.name
            WHERE i.created_at >= ? AND (? OR i.done = false)
            GROUP BY i.id
            ORDER BY i.created_at DESC;
            "#,
        )
        .bind(cutoff_date_time)
        .bind(include_done)
        .fetch_all(executor)
        .await
        .map_err(|e| Error::SelectError("items", e))
    }

    pub async fn set_done(
        id: i64,
        done: bool,
        executor: impl Executor<'_, Database = super::DB>,
    ) -> Result<(), Error> {
        sqlx::query!(
            r#"
                UPDATE items
                SET
                    done = ?
                WHERE id = ?
                "#,
            done,
            id
        )
        .execute(executor)
        .await
        .map_err(|e| Error::UpdateError("items", e))
        .map(|_| ())
    }

    pub async fn get_by_id(
        id: i64,
        executor: impl Executor<'_, Database = super::DB>,
    ) -> Result<Option<Self>, Error> {
        match sqlx::query_as!(Item, "SELECT * FROM items where id = ?1", id)
            .fetch_one(executor)
            .await
        {
            Ok(item) => Ok(Some(item)),
            Err(err) => match err {
                sqlx::Error::RowNotFound => Ok(None),
                _ => Err(Error::SelectError("items", err)),
            },
        }
    }

    pub async fn all_with_tags(
        tags: &[&str],
        executor: impl Executor<'_, Database = super::DB>,
    ) -> Result<Vec<Self>, Error> {
        let sql = format!(
            r#"
		SELECT i.*
		FROM items i
		JOIN items_to_tags st ON s.id = st.item_id
		JOIN tags t ON st.tag_id = t.name
		WHERE t.name IN ({})
		GROUP BY s.id
		HAVING COUNT(DISTINCT t.name) = {};
		"#,
            tags.iter()
                .map(|_| "?")
                .intersperse(",")
                .collect::<Box<str>>(),
            tags.len() // Should be safe to directly interpolate into query
        );
        let mut query = sqlx::query_as(&sql);

        for tag in tags {
            query = query.bind(tag);
        }

        query
            .fetch_all(executor)
            .await
            .map_err(|e| Error::SelectError("items", e))
    }

    /// Inserts self into the database and populates its `id` field
    pub async fn insert(
        &mut self,
        executor: impl Executor<'_, Database = super::DB>,
    ) -> Result<(), Error> {
        let id = sqlx::query!(
            r#"
		INSERT OR IGNORE INTO items(link, title, description, author, published, source_link, image, favorite, source_id)
		VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
		"#,
            self.link,
            self.title,
            self.description,
            self.author,
            self.published,
            self.source_link,
			self.image,
			self.favorite,
			self.source_id
        )
        .execute(executor)
        .await
        .map_err(|e| Error::InsertError("items", e))?
        .last_insert_rowid();

        self.id = id;
        Ok(())
    }

    pub async fn add_tags(
        id: i64,
        tags: &[&str],
        executor: impl Executor<'_, Database = super::DB>,
    ) -> Result<(), Error> {
        let sql = format!(
            r#"
			INSERT OR IGNORE INTO items_to_tags (item_id, tag_id)
			SELECT {}, name
			FROM tags
			WHERE name IN ({});
		"#,
            id,
            tags.into_iter()
                .map(|_| "?")
                .intersperse(",")
                .collect::<Box<str>>()
        );

        let mut query = sqlx::query(&sql);

        for tag in tags {
            query = query.bind(tag);
        }

        query
            .execute(executor)
            .await
            .map_err(|e| Error::InsertError("items_to_tags", e))
            .map(|_| ())
    }

    pub async fn remove_tag(
        id: i64,
        tag: &str,
        executor: impl Executor<'_, Database = super::DB>,
    ) -> Result<(), Error> {
        sqlx::query!(
            "DELETE FROM items_to_tags WHERE item_id = ? AND tag_id = ?",
            id,
            tag
        )
        .execute(executor)
        .await
        .map_err(|e| Error::DeleteError("items_to_tags", e))
        .map(|_| ())
    }

    pub async fn delete(
        id: i64,
        executor: impl Executor<'_, Database = super::DB>,
    ) -> Result<(), Error> {
        sqlx::query!("DELETE FROM items WHERE id = ?", id)
            .execute(executor)
            .await
            .map_err(|e| Error::DeleteError("tags", e))
            .map(|_| ())
    }
}
