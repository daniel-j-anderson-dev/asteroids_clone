use axum::Router;
use std::{
    path::Path,
    process::{Command, Output},
};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // is the wasm binary already in place?
    if !Path::new("./asteroids.wasm").exists() {
        // is the wasm binary already at the default build location? (path relative to this file)
        if !Path::new("./../../target/wasm32-unknown-unknown/release/asteroids.wasm").exists() {
            // build wasm in default build location if it's not there

            println!("Checking for wasm32-unknown-unknown target; please wait...");
            let output = Command::new("rustup")
                .args(&["target", "add", "wasm32-unknown-unknown"])
                .output()
                .expect("Failed to add wasm target. is rustup installed?");
            print_output("wasm target is available", output);

            println!("Building wasm; please wait...");
            let output = Command::new("cargo")
                .args(&["build", "--release", "--target", "wasm32-unknown-unknown"])
                .output()
                .expect("Failed to build wasm binary.");
            print_output("wasm binary built!", output);
        }

        // copy the wasm binary from the default build location to this example directory (path relative to Cargo.toml)
        std::fs::copy(
            "./target/wasm32-unknown-unknown/release/asteroids.wasm",
            "./examples/wasm/asteroids.wasm",
        )
        .expect("Failed to copy wasm binary to example folder");
    }

    println!("wasm binary is in built and in place\nstarting server...");

    let listener = TcpListener::bind("127.0.0.1:5000")
        .await
        .expect("failed to bind tcp listener");
    let files = ServeDir::new("./examples/wasm/");
    let routes = Router::new().nest_service("/", files);
    
    println!("hosting wasm on http://localhost:5000");
    axum::serve(listener, routes).await.expect("server error");
}

fn print_output(message: &str, output: Output) {
    println!(
        "{}\n{}\nstdout\n{}\nstderr\n{}",
        message,
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
