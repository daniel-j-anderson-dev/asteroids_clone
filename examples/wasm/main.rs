use axum::Router;
use std::path::Path;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if Path::new("./asteroids.wasm").exists() {
        panic!("please build the wasm binary and paste it at ./examples/wasm/asteroids.wasm\nsee readme.md");
    }
    let listener = TcpListener::bind("::1:5000").await?;
    let files = ServeDir::new("./examples/wasm/");
    let routes = Router::new().nest_service("/", files);
    println!("http://localhost:5000");
    axum::serve(listener, routes).await?;
    return Ok(());
}
