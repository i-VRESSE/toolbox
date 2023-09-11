pub mod dev;
use axum::{routing::get, Router};

use tower_http::trace::{self, TraceLayer};
use tracing::Level;

pub fn init_router() -> Router {
    Router::new().route("/ping", get(dev::ping)).layer(
        TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
            .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
    )
}
