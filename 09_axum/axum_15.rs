use std::sync::{Arc,Mutex};

use axum::{Json, Router, extract::{Path, State}, response::IntoResponse, routing::{delete, get, post, put}};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[tokio::main]

async fn main() {
    let list = Arc::new(Mutex::new(PersoneList{
        list: vec![],
    }));
    let app = app(list);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    if let Err(e) = open::that("http://127.0.0.1:8080/") {
        eprintln!("Can not open browser: {}", e);
    }

    axum::serve(listener, app).await.unwrap();
}

fn app(list: Arc<Mutex<PersoneList>> ) -> Router {
    Router::new().route("/", get(|| async { "Home page"}))
    .route("/list", get(get_list))
    .route("/persone/{id}", get(get_singl))
    .route("/add_persone", post(add_new_persone))
    .route("/remove_persone/{id}", delete(remove_persone))
    .route("/update_persone/{id}", put(update_persone))
    .with_state(list)
}

async fn get_list(State(list): State<Arc<Mutex<PersoneList>>>) -> impl IntoResponse{
    let persone_list= list.lock().unwrap().clone();
    let json_data = serde_json::to_string_pretty(&persone_list).unwrap();
    (StatusCode::OK,json_data).into_response()
}

async fn get_singl(State(list): State<Arc<Mutex<PersoneList>>>, Path(id): Path<u64>) -> impl IntoResponse{
    let persone_list= list.lock().unwrap().clone();
    match persone_list.list.iter().find(|persone| persone.id == id){
        None => (StatusCode::NOT_FOUND,"Persone Not Found".to_string()).into_response(),
        Some(persone) => {
            let json_data = serde_json::to_string_pretty(&persone).unwrap();
            (StatusCode::OK,json_data).into_response()
        }
    }
}

async fn add_new_persone (State(list): State<Arc<Mutex<PersoneList>>>, Json(persone_request): Json<PersonRequest>) -> impl IntoResponse{
    let mut persone_list= list.lock().unwrap();
    let new_persone = Persone{id: if persone_list.list.len() == 0 {0} else {persone_list.list.last().unwrap().id +1}, name: persone_request.name};
    persone_list.list.push(new_persone);
    (StatusCode::OK,"Add New Persone".to_string()).into_response()
}

async fn remove_persone (State(list): State<Arc<Mutex<PersoneList>>>, Path(id): Path<u64>) -> impl IntoResponse{
    let mut persone_list= list.lock().unwrap();
    match persone_list.list.iter().find(|persone| persone.id == id){
        None => (StatusCode::NOT_FOUND,"Persone Not Found".to_string()).into_response(),
        Some(_) => {
            let new_list = persone_list.list.iter().filter(|persone| persone.id != id)
                                         .map(|persone| persone.clone()).collect::<Vec<Persone>>();
            persone_list.list = new_list;
            (StatusCode::OK,"Persone Removed".to_string()).into_response()
        }
    }
}

async fn update_persone(State(list): State<Arc<Mutex<PersoneList>>>,Path(id): Path<u64>,Json(person_request): Json<PersonRequest>,) -> impl IntoResponse {
    let mut persone_list = list.lock().unwrap();
    match persone_list.list.iter_mut().find(|persone| persone.id == id) {
        None => (StatusCode::NOT_FOUND, "Persone Not Found".to_string()).into_response(),
        Some(persone) => {
            persone.name = person_request.name.clone();
            (StatusCode::OK, "Persone Updated".to_string()).into_response()
        }
    }
}
#[derive(Debug,PartialEq,Clone, Serialize, Deserialize)]
struct Persone{
    id: u64,
    name: String,
}

#[derive(Debug,PartialEq,Clone, Serialize, Deserialize)]
struct PersoneList{
    list: Vec<Persone>,
}

#[derive(Deserialize)]
struct PersonRequest{
    name: String,
}