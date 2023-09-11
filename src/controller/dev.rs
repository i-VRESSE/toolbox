use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Message {
    message: String,
}

pub async fn ping() -> Json<Message> {
    let message = Message {
        message: "pong".to_string(),
    };
    Json(message)
}
