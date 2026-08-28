use std::{collections::HashMap, sync::{Arc,Mutex}};

use axum::{Json, Router, body::Body, extract::{Path, Query, Request, State}, response::{IntoResponse, Response}, routing::{delete, get, patch, post,put}};
use reqwest::StatusCode;
use serde_json::{Value, json, to_string_pretty};
use serde::{Deserialize, Serialize};

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
    let shared_state = Arc::new(Mutex::new(
        PersoneRequest{name: "Jim".to_string(), age: 20}
    ));
    Router::new()
        .route("/", get(reqwest))
        .route("/hello", get(hello))
        .route("/world/{id}", get(world))
        .route("/hello_2", get(hello_2))
        .with_state(shared_state)
}

async fn hello() -> Response{
    let person = PersoneRequest { name: "Syslik".to_string(), age: 4};
    let json_data = to_string_pretty(&person).unwrap();
    Response::new(Body::new(json_data))
}

async fn hello_2(State(person): State<Arc<Mutex<PersoneRequest>>>) -> impl IntoResponse{
    let mut person = person.lock().unwrap();
    println!("{:?}",person);
    (*person).name = "Tom".to_string();
    (*person).age = 20;
    (StatusCode::ACCEPTED, "Aloha")
}

async fn world(State(person): State<Arc<Mutex<PersoneRequest>>>) -> String {
    println!("{:?}",person);

    "Test_2".to_string()
}

async fn reqwest(req: Request) -> &'static str {
    let headers = req.headers();
    let method = req.method();
    let uri = req.uri();

    println!("Headers: {:?}",headers);
    println!("Methods: {:?}",method);
    println!("URI: {:?}",uri);
    
    "Home"
}

#[derive(Debug,Deserialize,Serialize)]
struct PersoneRequest{
    name: String,
    age: u32,
}