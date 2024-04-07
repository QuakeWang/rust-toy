use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tower_http::trace::TraceLayer;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .route("/greet/:name", get(greet))
        .layer(TraceLayer::new_for_http())
        .fallback(handler_404);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3035")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello world!</h1>")
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "404 Not Found")
}

async fn greet(Path(name): Path<String>) -> impl IntoResponse {
    (HelloTemplate { name }).to_string()
}
