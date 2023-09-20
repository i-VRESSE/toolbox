use axum::Router;
use serde::{Deserialize, Serialize};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

#[derive(Deserialize)]
pub struct RequestBody {
    pub payload: String,
}

#[derive(Serialize)]
pub struct Message {
    pub output: String,
}

pub fn make_router() -> Router {
    tracing_subscriber::fmt()
        .with_target(false)
        // .compact()
        .init();
    Router::new().layer(
        TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
            .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_router() {
        let result = std::panic::catch_unwind(make_router);
        assert!(result.is_ok());
    }
}

pub mod utils {
    use std::env;
    use tempfile;

    #[allow(deprecated)]
    // Decode and extract the contents of the zip file into a temporary directory.
    pub fn uncompress_payload(payload: &String) -> tempfile::TempDir {
        // Decode the `payload` field from the request body.
        let input = base64::decode(payload).unwrap();

        // Make a zip archive from the decoded input.
        match zip::ZipArchive::new(std::io::Cursor::new(input)) {
            Ok(mut archive) => {
                let temp_dir = tempfile::Builder::new().tempdir().unwrap();
                archive.extract(temp_dir.path()).unwrap();
                temp_dir
            }
            Err(_) => {
                panic!("This is not an archive")
            }
        }
    }

    pub fn run_command(command: &str, input_data_dir: &tempfile::TempDir) -> (String, String) {
        // Run a command inside this folder and return the output.
        let output = std::process::Command::new(command)
            .current_dir(input_data_dir.path())
            .output()
            .expect("failed to execute process");

        // Return the output of the command.
        let stdout = String::from_utf8(output.stdout).unwrap();

        // return the error of the command.
        let stderr = String::from_utf8(output.stderr).unwrap();

        // Combine both in a log file
        (stdout, stderr)
    }

    pub fn get_sys_var(var: &str) -> String {
        
        match env::var(var) {
            Ok(val) => val,
            Err(_) => panic!("{} is not set or an error occurred.", var),
        }
    }

    pub fn get_port() -> u16 {
        let var: &str = "PORT";
        let port = match env::var(var) {
            Ok(val) => val,
            Err(_) => panic!("{} is not set or an error occurred.", var),
        };
        port.parse::<u16>().unwrap()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::File;
        use std::io::Write;

        #[test]
        fn test_run_command() {
            let temp_dir = tempfile::Builder::new().tempdir().unwrap();
            let file_path = temp_dir.path().join("test.txt");
            let mut file = File::create(file_path).unwrap();
            writeln!(file, "test").unwrap();

            let (stdout, stderr) = run_command("ls", &temp_dir);
            assert!(stdout.contains("test.txt"));
            assert_eq!(stderr, "");
        }

        #[test]
        #[should_panic]
        fn test_run_command_error() {
            let temp_dir = tempfile::Builder::new().tempdir().unwrap();
            run_command("invalid_command", &temp_dir);
        }

        #[test]
        fn test_get_sys_var() {
            std::env::set_var("TEST_VAR", "test_value");
            assert_eq!(get_sys_var("TEST_VAR"), "test_value");
        }

        #[test]
        fn test_uncompress_payload() {
            let payload = String::from("UEsDBAoAAAAAAE1YNFcAAAAAAAAAAAAAAAAOABwAaW5wdXRfZGF0YS50eHRVVAkAAyK1CmUitQpldXgLAAEE6AMAAAToAwAAUEsBAh4DCgAAAAAATVg0VwAAAAAAAAAAAAAAAA4AGAAAAAAAAAAAAKSBAAAAAGlucHV0X2RhdGEudHh0VVQFAAMitQpldXgLAAEE6AMAAAToAwAAUEsFBgAAAAABAAEAVAAAAEgAAAAAAA==");
            let temp_dir = uncompress_payload(&payload);
            assert_eq!(temp_dir.path().exists(), true);
        }

        #[test]
        #[should_panic]
        fn test_uncompress_payload_error() {
            let payload = String::from("IA==");
            let temp_dir = uncompress_payload(&payload);
            assert_eq!(temp_dir.path().exists(), true);
        }

        #[test]
        fn test_get_port() {
            std::env::set_var("PORT", "8080");
            assert_eq!(get_port(), 8080);
        }
    }
}
