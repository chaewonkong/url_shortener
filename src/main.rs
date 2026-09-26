use axum::{
    Router,
    routing::{get, post},
};
use redis::{RedisError, aio::ConnectionManager};

use url_shortener::{
    config::Config,
    handler::{get_url, set_url},
};

async fn get_con(redis_url: &str) -> Result<ConnectionManager, RedisError> {
    let client = redis::Client::open(redis_url).unwrap();

    ConnectionManager::new(client).await
}

#[tokio::main]
async fn main() {
    let config = Config::from_env().unwrap();

    let app = Router::new()
        // .route("/health", get(|| async { "ok" }))
        .route("/set", post(set_url))
        .route("/{id}", get(get_url))
        .with_state(get_con(&config.redis_url).await.unwrap());

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", config.server_port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
