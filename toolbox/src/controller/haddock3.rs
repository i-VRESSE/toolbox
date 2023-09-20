use axum::response::{IntoResponse, Json, Response};
use core::panic;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;

#[derive(Deserialize, Serialize)]
pub struct RequestBody {
    payload: String,
}

#[derive(Serialize)]
pub struct Message {
    output: String,
}

// Redirect the request to `some_tool` endpoint
pub async fn haddock3_int_rescore(body: axum::extract::Json<RequestBody>) -> Response {
    // Get the endpoint from the environment variable.
    let endpoint = match env::var("HADDOCK3_INT_RESCORE_ENDPOINT") {
        Ok(val) => val,
        Err(_) => panic!("HADDOCK3_INT_RESCORE_ENDPOINT is not set or an error occurred."),
    };
    // Initialize a client
    let client = reqwest::Client::new();

    // Make a json request body
    let request_body = json!({
        "payload": body.payload
    });

    // Post the request to the endpoint
    let res = client
        .post(endpoint)
        .json(&request_body)
        .send()
        .await
        .unwrap();

    // --------------------------------------------------------------------------------
    // Handle the response, check status etc...
    let status = res.status();
    let message = Message {
        output: res.text().await.unwrap(),
    };

    if status != StatusCode::OK {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(message)).into_response()
    } else {
        (StatusCode::OK, Json(message)).into_response()
    }
    // --------------------------------------------------------------------------------
}
