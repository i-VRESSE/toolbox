use axum::response::{IntoResponse, Response};
use axum::{routing::post, Router};
use serde::Deserialize;
use std::net::SocketAddr;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

// ================================================================================
// These should be configs
const COMMAND: &str = "haddock3-int_rescore";
const PORT: u16 = 9000;
// ================================================================================

#[derive(Deserialize)]
struct RequestBody {
    payload: String, // base64 encoded zip file.
}

// Handle the POST request
async fn handle_post(body: axum::extract::Json<RequestBody>) -> Response {
    // Uncompress the payload
    let _input_data_dir = uncompress_payload(&body.payload).await;

    // Run a command inside this folder and return the output.
    let output = std::process::Command::new(COMMAND)
        // .current_dir(input_data_dir.path())
        .output()
        .expect("failed to execute process");

    // Return the output of the command.
    let stdout = String::from_utf8(output.stdout).unwrap();

    // return the error of the command.
    let stderr = String::from_utf8(output.stderr).unwrap();

    // Combine both in a log file
    format!("{}\n{}", stdout, stderr).into_response()
}

#[allow(deprecated)]
// Decode and extract the contents of the zip file into a temporary directory.
async fn uncompress_payload(payload: &String) -> tempfile::TempDir {
    // Decode the `payload` field from the request body.
    let input = base64::decode(payload).unwrap();

    // Make a zip archive from the decoded input.
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(input)).unwrap();

    // Create a temporary directory
    let temp_dir = tempfile::Builder::new().tempdir().unwrap();

    // Extract the contents of the zip archive into the temporary directory.
    archive.extract(temp_dir.path()).unwrap();

    // Return the temporary directory.
    temp_dir
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        // .compact()
        .init();

    let app = Router::new().route("/", post(handle_post)).layer(
        TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
            .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
    );

    println!("[EXECUTOR] Listening on port {}...", PORT);

    let addr = SocketAddr::from(([0, 0, 0, 0], PORT));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
