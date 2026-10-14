use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use redis::{AsyncCommands, RedisError, Script, aio::ConnectionManager};
use serde::Deserialize;

use crate::base62::{DecodeError, EncodeError, decode, encode};

#[derive(Debug)]
pub enum AppError {
    Redis(RedisError),
    NotFound,
    Base62EncodeError(EncodeError),
    Base62DecodeError(DecodeError),
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
            AppError::Base62EncodeError(e) => {
                eprintln!("encode error: {e:?}");
                (StatusCode::INTERNAL_SERVER_ERROR, "encode error").into_response()
            }
            AppError::Base62DecodeError(e) => {
                eprintln!("decode error: {e:?}");
                (StatusCode::INTERNAL_SERVER_ERROR, "decode error").into_response()
            }
        }
    }
}

const SET_WITH_ID: &str = r#"
    local offset_key, key_prefix = unpack(KEYS)
    local value = ARGV[1]

    if not offset_key or not key_prefix or not value then
        return redis.error_reply('ERR expected keys or value not provided')
    end

    local id = redis.call('INCR', offset_key)
    redis.call('SET', key_prefix .. ':' .. id, value)

    return id
"#;

const OFFSET_KEY: &str = "offset";
const KEY_PREFIX: &str = "short";

fn url_key(id: u64) -> String {
    return format!("{}:{}", KEY_PREFIX, id);
}

pub async fn get_url(
    Path(id): Path<String>,
    State(mut con): State<ConnectionManager>,
) -> Result<Redirect, AppError> {
    let decoded_id = decode(&id).map_err(|e| AppError::Base62DecodeError(e))?;
    let result = con.get::<String, String>(url_key(decoded_id)).await?;
    let uri = format!("https://{}", result);

    Ok(Redirect::temporary(&uri))
}

pub async fn set_url(
    State(mut con): State<ConnectionManager>,
    Json(url): Json<SetUrl>,
) -> Result<String, AppError> {
    // con.set::<_, _, ()>(&id, url.url).await?;
    let set_script = Script::new(SET_WITH_ID);

    let id: u64 = set_script
        .key(OFFSET_KEY)
        .key(KEY_PREFIX)
        .arg(url.url)
        .invoke_async(&mut con)
        .await?;

    encode(id).map_err(|e| AppError::Base62EncodeError(e))
}
