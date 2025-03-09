use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::prelude::*;

use super::{Error, Tag};

#[derive(Debug, FromRow, Serialize, Deserialize, ts_rs::TS)]
#[ts(export, export_to = "../web/src/types/Source.ts")]
pub struct Source {
    #[ts(type = "number")]
    pub id: i64,

    pub name: String,

    pub url: String,

    #[serde(skip_deserializing)]
    pub last_pub: chrono::NaiveDateTime,

    pub last_poll: Option<chrono::NaiveDateTime>,

    #[ts(type = "number | null")]
    pub ttl: Option<i64>,

    #[serde(default)]
    pub favorite: bool,

    #[serde(skip_deserializing)]
    pub created_at: chrono::NaiveDateTime,

    #[serde(skip_deserializing)]
    pub updated_at: chrono::NaiveDateTime,
}

impl Source {
    pub async fn get_all(
        executor: impl Executor<'_, Database = super::DB>,
    ) -> Result<Vec<Self>, Error> {
        sqlx::query_as!(Source, "SELECT * FROM sources")
            .fetch_all(executor)
            .await
            .map_err(|e| Error::SelectError("sources", e))
    }

    pub async fn get_by_id(
        id: i64,
        executor: impl Executor<'_, Database = super::DB>,
    ) -> Result<Option<Self>, Error> {
        match sqlx::query_as!(Source, "SELECT * FROM sources where id = ?1", id)
            .fetch_one(executor)
            .await
        {
            Ok(source) => Ok(Some(source)),
            Err(err) => match err {
                sqlx::Error::RowNotFound => Ok(None),
                _ => Err(Error::SelectError("sources", err)),
            },
        }
    }

    pub async fn all_with_tags(
        tags: &[&str],
        executor: impl Executor<'_, Database = super::DB>,
    ) -> Result<Vec<Self>, Error> {
        let sql = format!(
            r#"
		SELECT s.*
		FROM sources s
		JOIN sources_to_tags st ON s.id = st.source_id
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
            .map_err(|e| Error::SelectError("sources", e))
    }

    /// Inserts self into the database and populates its `id` field
    pub async fn insert(
        &mut self,
        executor: impl Executor<'_, Database = super::DB>,
    ) -> Result<(), Error> {
        let id = sqlx::query!(
            r#"
		INSERT INTO sources(name, url, last_pub, last_poll, ttl, favorite)
		VALUES (?1, ?2, ?3, ?4, ?5, ?6)
		"#,
            self.name,
            self.url,
            self.last_pub,
            self.last_poll,
            self.ttl,
            self.favorite
        )
        .execute(executor)
        .await
        .map_err(|e| Error::InsertError("sources", e))?
        .last_insert_rowid();

        self.id = id;
        Ok(())
    }

    pub async fn update(
        &mut self,
        executor: impl Executor<'_, Database = super::DB>,
    ) -> Result<(), Error> {
        sqlx::query!(
            r#"
		UPDATE sources
        SET
            name = ?1,
            url = ?2,
            last_pub = ?3,
            last_poll = ?4,
            ttl = ?5,
            favorite = ?6
		WHERE id = ?7
		"#,
            self.name,
            self.url,
            self.last_pub,
            self.last_poll,
            self.ttl,
            self.favorite,
            self.id
        )
        .fetch_optional(executor)
        .await
        .map_err(|e| Error::UpdateError("sources", e))?;

        Ok(())
    }

    pub async fn add_tags(
        id: i64,
        tags: &[&str],
        executor: impl Executor<'_, Database = super::DB>,
    ) -> Result<(), Error> {
        let sql = format!(
            r#"
			INSERT INTO sources_to_tags (source_id, tag_id)
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
            .map_err(|e| Error::InsertError("sources_to_tags", e))
            .map(|_| ())
    }

    pub async fn remove_tag(
        id: i64,
        tag: &str,
        executor: impl Executor<'_, Database = super::DB>,
    ) -> Result<(), Error> {
        sqlx::query!(
            "DELETE FROM sources_to_tags WHERE source_id = ? AND tag_id = ?",
            id,
            tag
        )
        .execute(executor)
        .await
        .map_err(|e| Error::DeleteError("sources_to_tags", e))
        .map(|_| ())
    }

    pub async fn tags(
        id: i64,
        executor: impl Executor<'_, Database = super::DB>,
    ) -> Result<Vec<Tag>, Error> {
        sqlx::query_as!(
            Tag,
            r#"
        SELECT t.*
        FROM tags t
        JOIN sources_to_tags st ON t.name = st.tag_id
        WHERE st.source_id = ?;
        "#,
            id
        )
        .fetch_all(executor)
        .await
        .map_err(|e| Error::SelectError("sources_to_tags", e))
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
