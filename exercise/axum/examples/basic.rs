use std::net::SocketAddr;
use axum::{Json, Router, Server};
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};

use jsonwebtoken as jwt;

const SECRET: &[u8] = b"deadbeef";

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: usize,
    pub user_id: usize,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTodo {
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginResponse {
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: usize,
    name: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/todos", get(todos_handler)
            .post(create_todos_handler))
        .route("/login", post(login_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on http://{}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index_handler() -> Html<&'static str> {
    Html("Hello world!")
}

async fn todos_handler() -> Json<Vec<Todo>> {
    Json(vec![
        Todo {
            id: 1,
            user_id: 1,
            title: "Todo 1".to_string(),
            completed: false,
        },
        Todo {
            id: 2,
            user_id: 2,
            title: "Todo 2".to_string(),
            completed: false,
        },
    ])
}

async fn create_todos_handler(Json(_todo): Json<CreateTodo>) -> StatusCode {
    StatusCode::CREATED
}

async fn login_handler(Json(login): Json<LoginRequest>) -> Json<LoginResponse> {
    let claims = Claims {
        id: 1,
        name: "QuakeWang".to_string(),
    };
    let key = jwt::EncodingKey::from_secret(SECRET);
    let token = jwt::encode(&jwt::Header::default(), &claims, &key).unwrap();
    Json(LoginResponse { token })
}