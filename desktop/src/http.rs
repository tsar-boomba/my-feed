use serde::de::DeserializeOwned;

pub async fn request<T: Send + DeserializeOwned + 'static>(uri: String) -> anyhow::Result<T> {
    blocking::unblock(|| Ok(ureq::get(uri).call()?.into_body().read_json::<T>()?)).await
}
