use std::collections::HashMap;

use axum::{Router, extract::{Path, Query}, routing::{delete, get, patch, post,put}};

#[tokio::main] 
async fn main(){
    let app = app();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    let addr = listener.local_addr().unwrap();
    println!("Server is listening on {}", addr);

    if let Err(e) = open::that(format!("http://127.0.0.1:{}/", addr.port())) {
        eprintln!("Can not open browser: {}", e);
    }

    axum::serve(listener, app).await.unwrap();
}
fn app() -> Router {
    Router::new().route("/", get(|| async {"Home"}))
        .route("/hello", get(hello))
        .route("/world/{id}", get(world))
}

async fn hello(Query(params):Query<HashMap<String, String>>) -> &'static str{
    for key in params.keys(){
        println!("Key: {key}");
    }
    for v in params.values(){
        println!("Value: {v}");
    }
    "Hi!"
}

async fn world(Path(id): Path<i32>) -> String {
    id.to_string()
}