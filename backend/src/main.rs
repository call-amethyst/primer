use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let api = Router::new().route("/hello", get(hello));

    let app = Router::new().nest("/api", api);
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn hello() -> &'static str {
    "Hello World from the backend!"
}
