use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::prelude::*;

use super::Error;

#[derive(Debug, FromRow, Serialize, Deserialize, ts_rs::TS)]
#[ts(export, export_to = "../web/src/types/Tag.ts")]
pub struct Tag {
    pub name: String,
    pub background_color: Option<String>,
    pub text_color: Option<String>,
    pub border_color: Option<String>,
    #[serde(skip_deserializing)]
    pub created_at: chrono::NaiveDateTime,
    #[serde(skip_deserializing)]
    pub updated_at: chrono::NaiveDateTime,
}

impl Tag {
    pub async fn get_all(
        executor: impl Executor<'_, Database = super::DB>,
    ) -> Result<Vec<Self>, Error> {
        sqlx::query_as!(Tag, "SELECT * FROM tags")
            .fetch_all(executor)
            .await
            .map_err(|e| Error::SelectError("tags", e))
    }

    pub async fn get_by_name(
        name: &str,
        executor: impl Executor<'_, Database = super::DB>,
    ) -> Result<Option<Self>, Error> {
        match sqlx::query_as!(Tag, "SELECT * FROM tags where name = ?1", name)
            .fetch_one(executor)
            .await
        {
            Ok(tag) => Ok(Some(tag)),
            Err(err) => match err {
                sqlx::Error::RowNotFound => Ok(None),
                _ => Err(Error::SelectError("tags", err)),
            },
        }
    }

    /// Inserts self into the database
    pub async fn insert(
        &mut self,
        executor: impl Executor<'_, Database = super::DB>,
    ) -> Result<(), Error> {
        sqlx::query!(
            r#"
		INSERT INTO tags(name, background_color, text_color, border_color)
		VALUES (?1, ?2, ?3, ?4)
		"#,
            self.name,
            self.background_color,
            self.text_color,
            self.border_color,
        )
        .execute(executor)
        .await
        .map_err(|e| Error::InsertError("tags", e))?;

        Ok(())
    }

    pub async fn insert_many(
        tags: &[Self],
        executor: impl Executor<'_, Database = super::DB>,
    ) -> Result<(), Error> {
        let sql = format!(
            r#"
		INSERT OR IGNORE INTO tags(name, background_color, text_color, border_color)
		VALUES {};
		"#,
            tags.iter()
                .map(|_| "(?, ?, ?, ?)")
                .intersperse(",")
                .collect::<Box<str>>()
        );
        let mut query = sqlx::query(&sql);

        for tag in tags {
            query = query
                .bind(&tag.name)
                .bind(&tag.background_color)
                .bind(&tag.text_color)
                .bind(&tag.background_color);
        }

        query
            .execute(executor)
            .await
            .map_err(|e| Error::InsertError("tags", e))
            .map(|_| ())
    }

    pub async fn delete(
        name: &str,
        executor: impl Executor<'_, Database = super::DB>,
    ) -> Result<(), Error> {
        sqlx::query!("DELETE FROM tags WHERE name = ?", name)
            .execute(executor)
            .await
            .map_err(|e| Error::DeleteError("tags", e))
            .map(|_| ())
    }
}
