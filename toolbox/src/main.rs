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
    println!("Listening on port 80...");

    let addr = SocketAddr::from(([0, 0, 0, 0], 80));

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}