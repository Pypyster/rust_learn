use std::{cmp::Ordering::Greater, collections::HashMap, sync::{Arc,Mutex}};
use async_std::stream::StreamExt;
use tokio_util::codec::{BytesCodec, FramedRead};
use tower_http::services::ServeDir;
use axum::{Json, Router, body::Body, extract::{Extension, Form, Path, Query, Request, State}, http::{ Uri}, middleware::{Next, from_fn_with_state}, response::{IntoResponse, Redirect, Response}, routing::{delete, get, patch, post,put}};
use reqwest::StatusCode;
use serde_json::{Value, json, to_string_pretty};
use serde::{Deserialize, Serialize};
use axum::middleware::from_fn;

#[tokio::main] 
async fn main(){
    let app = app();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    let addr = listener.local_addr().unwrap();
    println!("Server is listening on {}", addr);

    // if let Err(e) = open::that(format!("http://127.0.0.1:{}/", addr.port())) {
    //     eprintln!("Can not open browser: {}", e);
    // }
    

    axum::serve(listener, app).await.unwrap();
}
fn app() -> Router {
    let user = User{
        name: "Tom".to_string(),
        age: 18,
    };

    let animal = Animal{
        name: "Flafu".to_string(),
        wool: false,
    };

    let state = Arc::new(Message{ content: "Test!".to_string()});

    let static_file = ServeDir::new("./asserts");
    Router::new().route("/", get(|| async {"Home page"}))
    .route("/hello", get(hello))
    .route("/hello_2", get(hello_2))
    .route("/text", get(text))
    .layer(from_fn_with_state(state.clone(), mid))
    .nest_service("/static", static_file)
    .with_state((user,animal))
}

async fn hello(State((user,animal)): State<(User,Animal)>) -> impl IntoResponse {
    let msg = format!(
        "The {} at {} y.o. has a {} with bool: {}",
        user.name, user.age, animal.name, animal.wool
    );
        (StatusCode::OK, msg).into_response()
}

async fn hello_2() -> impl IntoResponse{
    User{
        name: "Tobby".to_string(),
        age: 35,
    }
}
async fn mid(State(state): State<Arc<Message>>, req:Request, next: Next) -> impl IntoResponse {
    println!("{:?}", state);
    let req = next.run(req).await;
    req
}

async fn text() -> impl IntoResponse {
    let file = tokio::fs::File::open("Lx_8.0_UV.csv").await.unwrap();

    let stream = FramedRead::new(file, BytesCodec::new())
    .map(|r| r.map(|b| b.freeze()));

    Body::from_stream(stream)
}

impl IntoResponse for User{
    fn into_response(self) -> axum::response::Response {
        let res = serde_json::to_string(&self).unwrap();
        Response::new(Body::from(res))
    }
}

#[derive(Debug)]
struct Message{
    content: String,
}

#[derive(Clone, Serialize)]
struct User{ 
    name: String,
    age: u16,
}

#[derive(Clone)]
struct Animal{ 
    name: String,
    wool: bool,
}