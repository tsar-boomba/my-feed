use freya::query::QueryCapability;

use crate::{http, models::{GetItemsReturn, Item}};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GetItems;

impl QueryCapability for GetItems {
    type Ok = Vec<GetItemsReturn>;

    type Err = anyhow::Error;

    type Keys = ();

    async fn run(&self, _keys: &Self::Keys) -> Result<Self::Ok, Self::Err> {
        let res = http::request::<Vec<GetItemsReturn>>("https://my.igamble.dev/api/items?from_last=2w".into()).await?;
        Ok(res)
    }
}
