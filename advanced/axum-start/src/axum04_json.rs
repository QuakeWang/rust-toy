use axum::{
    extract::Query,
    response::Html,
    routing::{get, post},
    Form, Json, Router,
};
use serde::Deserialize;
use tower_http::trace::TraceLayer;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Params {
    foo: Option<i32>,
    bar: Option<String>,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .route("/query", get(query))
        .route("/form", get(show_form).post(accept_form))
        .route("/json", post(accept_json))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3033")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello world!</h1>")
}

async fn query(Query(params): Query<Params>) -> Html<&'static str> {
    tracing::debug!("Query params: {:?}", params);
    Html("<h3>Test query</h3>")
}

async fn show_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/form" method="post">
                    <label for="name">
                        Enter your name:
                        <input type="text" name="name">
                    </label>

                    <label>
                        Enter your email:
                        <input type="text" name="email">
                    </label>

                    <input type="submit" value="Subscribe!">
                </form>
            </body>
        </html>
        "#,
    )
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Input {
    name: String,
    email: String,
}

async fn accept_form(Form(input): Form<Input>) -> Html<&'static str> {
    tracing::debug!("form params {:?}", input);
    Html("<h3>Form posted</h3>")
}

async fn accept_json(Json(input): Json<Input>) -> Html<&'static str> {
    tracing::debug!("json params {:?}", input);
    Html("<h3>Json posted</h3>")
}