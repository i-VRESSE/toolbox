use axum::routing::post;
use std::net::SocketAddr;
use toolbox::make_router;

mod controller;

#[tokio::main]
async fn main() {
    let port = toolbox::utils::get_port();
    let router = make_router().route("/rescore", post(controller::haddock3::rescore));
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Listening on port {}...", port);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
