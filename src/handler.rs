use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use redis::{AsyncCommands, RedisError, aio::ConnectionManager};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug)]
pub enum AppError {
    Redis(RedisError),
    NotFound,
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
            AppError::Redis(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal error").into_response()
            }
            AppError::NotFound => StatusCode::NOT_FOUND.into_response(),
        }
    }
}

pub async fn get_url(
    Path(id): Path<String>,
    State(mut con): State<ConnectionManager>,
) -> Result<Redirect, AppError> {
    let result = con.get::<String, String>(id.to_string()).await?;
    let uri = format!("https://{}", result);

    Ok(Redirect::temporary(&uri))
}

pub async fn set_url(
    State(mut con): State<ConnectionManager>,
    Json(url): Json<SetUrl>,
) -> Result<String, AppError> {
    let id = Uuid::now_v7().to_string();
    con.set::<_, _, ()>(&id, url.url).await?;

    Ok(id)
}
