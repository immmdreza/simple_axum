use axum::{routing::get, Router};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = ([127, 0, 0, 1], 3000).into();

    axum::Server::bind(&addr)
        .serve(
            // Here we can register more routes.
            Router::new()
                // `GET /` goes to `root`
                .route("/", get(root))
                .into_make_service(),
        )
        .await?;

    Ok(())
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
