use axum::{routing::get, Router};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let debug = false;
    let addr = (
        [127, 0, 0, 1],
        if debug {
            3000
        } else {
            iis::get_port().parse()?
        },
    )
        .into();

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
