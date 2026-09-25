use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
};
use redis::{AsyncCommands, RedisError, aio::ConnectionManager};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug)]
enum AppError {
    Redis(RedisError),
    NotFound,
}

#[derive(Debug, Deserialize)]
struct SetUrl {
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

async fn get_url(
    Path(id): Path<String>,
    State(mut con): State<ConnectionManager>,
) -> Result<Redirect, AppError> {
    let result = con.get::<String, String>(id.to_string()).await?;
    let uri = format!("https://{}", result);

    Ok(Redirect::temporary(&uri))
}

async fn set_url(
    State(mut con): State<ConnectionManager>,
    Json(url): Json<SetUrl>,
) -> Result<String, AppError> {
    let id = Uuid::now_v7().to_string();
    con.set::<_, _, ()>(&id, url.url).await?;

    Ok(id)
}

#[tokio::main]
async fn main() {
    let redis_url = std::env::var("REDIS_URL").unwrap_or("localhost:6379".into());
    let port = std::env::var("SERVER_PORT")
        .unwrap_or("3000".into())
        .parse::<u16>()
        .unwrap();

    let client = redis::Client::open(redis_url).unwrap();
    let con = ConnectionManager::new(client).await.unwrap();

    let app = Router::new()
        // .route("/health", get(|| async { "ok" }))
        .route("/set", post(set_url))
        .route("/{id}", get(get_url))
        .with_state(con);

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
