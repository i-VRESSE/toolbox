use axum::response::{IntoResponse, Json, Response};
use reqwest::StatusCode;
use serde_json::json;

// Redirect the request to `some_tool` endpoint
pub async fn rescore(body: axum::extract::Json<toolbox::RequestBody>) -> Response {
    // Get the endpoint from the environment variable.
    let endpoint = toolbox::utils::get_sys_var("HADDOCK3_INT_RESCORE_ENDPOINT");

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
    let message = toolbox::Message {
        output: res.text().await.unwrap(),
    };

    if status != StatusCode::OK {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(message)).into_response()
    } else {
        (StatusCode::OK, Json(message)).into_response()
    }
    // --------------------------------------------------------------------------------
}
