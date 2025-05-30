mod db;
mod router;

use axum::{
    Router,
    http::{StatusCode, Uri},
    routing::get,
};

#[tokio::main]
async fn main() {
    db::main();

    async fn fallback(uri: Uri) -> (StatusCode, String) {
        (StatusCode::NOT_FOUND, format!("No route for {uri}"))
    }

    use serde::Serialize;

    #[derive(Serialize)]
    struct Welcome {
        message: String,
    }

    async fn welcome() -> axum::Json<Welcome> {
        let msg = Welcome {
            message: "Welcome to rust".to_string(),
        };
        axum::Json(msg)
    }

    let app = Router::new()
        .route("/", get(welcome))
        .merge(router::main())
        .fallback(fallback);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
