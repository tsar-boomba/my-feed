use sqlx::Sqlite;
use thiserror::Error;

pub mod item;
pub mod source;
pub mod tag;

pub use item::Item;
pub use source::Source;
pub use tag::Tag;

type DB = Sqlite;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error inserting row into {0}: {1:?}")]
    InsertError(&'static str, sqlx::Error),
    #[error("Error updating row in {0}: {1:?}")]
    UpdateError(&'static str, sqlx::Error),
    #[error("Error selecting rows from {0}: {1:?}")]
    SelectError(&'static str, sqlx::Error),
    #[error("Error deleting rows in {0}: {1:?}")]
    DeleteError(&'static str, sqlx::Error),
    #[error("Row for {0} is invalid because \"{1}\"")]
    InvalidRow(&'static str, String),
}

impl Error {
    /// Panics if this is an invalid row error
    pub fn into_sqlx_error(self) -> sqlx::Error {
        match self {
            Error::InsertError(_, error) => error,
            Error::UpdateError(_, error) => error,
            Error::SelectError(_, error) => error,
            Error::DeleteError(_, error) => error,
            Error::InvalidRow(_, _) => panic!("No sqlx error"),
        }
    }
}
