use axum::Router;


#[tokio::main]
async fn main () -> Result<(), Box<dyn std::error::Error>> {
    if std::path::Path::new("./asteroids.wasm").exists() {
        panic!("please build the wasm binary and paste it at ./examples/wasm/asteroids.wasm\n");
    }
    let listener = tokio::net::TcpListener::bind("::1:5000").await?;
    let files = tower_http::services::ServeDir::new("./examples/wasm/");
    let routes = Router::new().nest_service("/", files) ;
    axum::serve(listener, routes).await?;
    return Ok(());
}
