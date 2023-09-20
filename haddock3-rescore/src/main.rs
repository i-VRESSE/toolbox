extern crate toolbox;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use std::net::SocketAddr;

// Handle the POST request
async fn handle_post(body: axum::extract::Json<toolbox::RequestBody>) -> Response {
    // Uncompress the payload
    let input_data_dir = toolbox::utils::uncompress_payload(&body.payload);

    let command = toolbox::utils::get_sys_var("COMMAND");

    // Run a command inside this folder and return the output.
    let (stdout, stderr) = toolbox::utils::run_command(&command, &input_data_dir);

    // Combine both in a log file
    format!("{}\n{}", stdout, stderr).into_response()
}

#[tokio::main]
async fn main() {
    let port = toolbox::utils::get_port();
    let app = toolbox::make_router().route("/", post(handle_post));
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Listening on port {}...", port);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
