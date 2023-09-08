use axum::response::{IntoResponse, Json, Response};
use axum::{extract::Query, routing::get, Router};
use rand::{thread_rng, Rng};
use serde::Deserialize;
use serde::Serialize;
use std::net::SocketAddr;

// `Deserialize` need be implemented to use with `Query` extractor.
#[derive(Deserialize)]
struct RangeParameters {
    start: usize,
    end: usize,
}

#[derive(Serialize)]
struct Result {
    result: usize,
}

async fn handler(Query(range): Query<RangeParameters>) -> Response {
    // Generate a random number in range parsed from query.
    let random_number = thread_rng().gen_range(range.start..range.end);

    // Send response in JSON format.
    let response = Result {
        result: random_number,
    };
    Json(response).into_response()
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
