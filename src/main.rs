use std::net::SocketAddr;

mod controller;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        // .compact()
        .init();

    let router = controller::init_router();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
