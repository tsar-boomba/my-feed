use rustc_hash::FxHashSet;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Item {
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

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct GetItemsReturn {
    #[serde(flatten)]
    pub item: Item,
    pub tags: FxHashSet<String>,
}
