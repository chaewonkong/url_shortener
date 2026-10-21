use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use redis::{AsyncCommands, RedisError, aio::ConnectionManager};
use serde::Deserialize;

use crate::random::random_string;

#[derive(Debug)]
pub enum AppError {
    Redis(RedisError),
    NotFound,
    CodeExhausted,
}

#[derive(Debug, Deserialize)]
pub struct SetUrl {
    url: String,
}

impl From<RedisError> for AppError {
    fn from(value: RedisError) -> Self {
        AppError::Redis(value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::Redis(e) => {
                eprintln!("redis error: {e:?}");
                (StatusCode::INTERNAL_SERVER_ERROR, "redis error").into_response()
            }
            AppError::NotFound => StatusCode::NOT_FOUND.into_response(),
            AppError::CodeExhausted => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "creation failed, retry later",
            )
                .into_response(),
        }
    }
}

const KEY_PREFIX: &str = "short";

fn url_key(id: &str) -> String {
    return format!("{}:{}", KEY_PREFIX, id);
}

pub async fn get_url(
    Path(id): Path<String>,
    State(mut con): State<ConnectionManager>,
) -> Result<Redirect, AppError> {
    let result = con.get::<String, String>(url_key(&id)).await?;
    let uri = format!("https://{}", result);

    Ok(Redirect::temporary(&uri))
}

const BASE_LEN: usize = 7;
const MAX_LEN: usize = 10;
const MAX_RETRY: usize = 3;

pub async fn set_url(
    State(mut con): State<ConnectionManager>,
    Json(url): Json<SetUrl>,
) -> Result<String, AppError> {
    for n in BASE_LEN..MAX_LEN {
        for _ in 0..MAX_RETRY {
            let id = random_string(n);
            let created: bool = con.set_nx(url_key(&id), &url.url).await?;
            if created {
                return Ok(id);
            }
        }
    }
    Err(AppError::CodeExhausted)
}
