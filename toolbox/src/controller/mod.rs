pub mod haddock3;
use axum::{routing::post, Router};

use tower_http::trace::{self, TraceLayer};
use tracing::Level;

pub fn init_router() -> Router {
    Router::new()
        .route(
            "/haddock3_int_rescore",
            post(haddock3::haddock3_int_rescore),
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
}
