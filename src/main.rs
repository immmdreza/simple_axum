use axum::{routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let port = std::env::var("ASPNETCORE_PORT").unwrap_or("4000".to_string());

    let app = 
            // Here we can register more routes.
            Router::new()
                // `GET /` goes to `root`
                .route("/", get(root))
                .into_make_service();

    let listener = TcpListener::bind(format!("127.0.0.1:{port}"))
        .await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
