use axum::extract;
use http::{HeaderMap, StatusCode};

use crate::{config::Config, ApiError};

use super::State;

pub async fn is_authorized(config: &Config, headers: &HeaderMap) -> Result<(), ApiError> {
	if headers.get("x-auth").is_some_and(|h| h.as_bytes() == config.password.as_bytes()) {
		Ok(())
	} else {
		Err(ApiError::Unauthorized)
	}
}

pub async fn login(extract::State(state): extract::State<State>, headers: HeaderMap) -> Result<(), ApiError> {
	is_authorized(&state.config, &headers).await
}
