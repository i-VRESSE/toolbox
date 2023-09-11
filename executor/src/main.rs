use axum::Json;
use axum::{routing::get, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

#[derive(Deserialize, Serialize)]
struct RequestBody {
    payload: String,
}

async fn handle_post(body: axum::extract::Json<RequestBody>) -> String {
    // The input is a base64 string, encoded from a zip file,
    //  decode it and extract the contents to a temporary directory.
    let input = base64::decode(&body.payload).unwrap();
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(input)).unwrap();
    // let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir = tempfile::Builder::new()
        .prefix("haddock3")
        .tempdir()
        .unwrap();

    archive.extract(temp_dir.path()).unwrap();

    // List the contents of the temporary directory.
    let mut contents = String::new();
    for entry in std::fs::read_dir(temp_dir.path()).unwrap() {
        let entry = entry.unwrap();
        contents.push_str(&format!("{}\n", entry.path().display()));
    }

    // Run a command called `haddock3` inside this folder
    //  and return the output.
    let output = std::process::Command::new("haddock3-int_rescore -r run1-ranairCDR-test -m 2")
        .current_dir(temp_dir.path())
        .output()
        .expect("failed to execute process");

    // Return the output of the command.
    let stdout = String::from_utf8(output.stdout).unwrap();

    // return the error of the command.
    let stderr = String::from_utf8(output.stderr).unwrap();

    // Combine both in a log file
    format!("{}\n{}", stdout, stderr)
}

#[derive(Serialize)]
pub struct Message {
    message: String,
}

pub async fn ping() -> Json<Message> {
    let message = Message {
        message: "pong".to_string(),
    };
    Json(message)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        // .compact()
        .init();

    let app = Router::new()
        .route("/ping", get(ping))
        .route("/", get(handle_post).post(handle_post))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
