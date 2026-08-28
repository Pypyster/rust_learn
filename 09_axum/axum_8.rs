use std::{cmp::Ordering::Greater, collections::HashMap, sync::{Arc,Mutex}};

use axum::{Json, Router, body::Body, extract::{Extension, Path, Query, Request, State,Form}, http::Uri, middleware::Next, response::{IntoResponse, Redirect, Response}, routing::{delete, get, patch, post,put}};
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
    let route1 = Router::new().route("/about", get(about));
    let share_state = ShareState{content: "Hello ALoha!".to_string()};
    
    let user_route = user_route();
    
    Router::new()
        .route("/", post(profile))       
        .route("/hello/{*rest}", get(hello).post(post_hello).put(put_hello).delete(delete_hello))
        .route("/world", get(world))
        .route("/rediarect", get(rediarect))
        .merge(route1)
        .route("/hello/world",get(about))
        .nest("/user", user_route)
        .fallback(not_found).layer(from_fn(reqwest_2)).with_state(Arc::new(share_state))
}

fn user_route() -> Router<Arc<ShareState>>{
    Router::new().route("/test", get(user_func))
}

async fn user_func(State(state): State<Arc<ShareState>>) -> impl IntoResponse{
    println!("{:?}", state);
    (StatusCode::OK, "Test State!").into_response()
}

async fn about(uri: Uri) -> impl IntoResponse{
    println!("URI: {:?}",uri);
    (StatusCode::OK,"Test about!").into_response()
}
async fn not_found() -> impl IntoResponse{
    (StatusCode::NOT_FOUND,"404 | Not found").into_response()
}
async fn hello(Path(path):Path<String>) -> impl IntoResponse{
    println!("path: {:?}",path);
    println!("You redeirect here!");
    (StatusCode::OK,"Test rest!").into_response()
}

async fn profile(Form(persone): Form<PersoneRequest>)-> impl IntoResponse {
    println!("{:?}", persone);
    (StatusCode::OK, "Test this").into_response()
}
async fn post_hello() -> impl IntoResponse{
    (StatusCode::OK, "Post hello").into_response()
}
async fn put_hello() -> impl IntoResponse{
    (StatusCode::OK, "Put hello")
}
async fn delete_hello() -> impl IntoResponse{
    (StatusCode::OK, "Delete hello")
}


async fn rediarect() -> impl IntoResponse{
    Redirect::to("/hello")
}
async fn world(Extension(persone): Extension<Arc<PersoneRequest>>) -> String{
    println!("{:?}", persone);
    "Test_2".to_string()
}    

async fn reqwest_2(mut req: Request,next: Next) -> impl IntoResponse {
    let persone = PersoneRequest{name: String::from("Merphy"), age: 0};
    req.extensions_mut().insert(Arc::new(persone));

    let response = next.run(req).await;
    response

}

#[derive(Debug,Deserialize,Serialize)]
struct PersoneRequest{
    name: String,
    age: u32,
}

#[derive(Debug)]
struct ShareState {
    content: String,
}