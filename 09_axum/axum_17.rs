use axum::{Json, Router, extract::{Path, State}, response::IntoResponse, routing::{delete, get, put, post}};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, pool, prelude::FromRow};

#[tokio::main] 
async fn main() {
    let app = app().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    let addr = listener.local_addr().unwrap();
    println!("Server is listening on {}", addr);

    if let Err(e) = open::that(format!("http://127.0.0.1:{}/", addr.port())) {
        eprintln!("Can not open browser: {}", e);
    }
    
    axum::serve(listener, app).await.unwrap();
}

async fn app() -> Router{
    let pool = db().await;
    Router::new()
    .route("/", get(|| async {"Home Page"}))
    .route("/todo_list", get(get_todo_list))
    .route("/add_todo", post(add_todo))
    .route("/todo/{id}", get(get_single))
    .route("/delete_todo/{id}", delete(delete_todo))
    .route("/update_todo/{id}", put(update_todo))
    .with_state(pool)
}

async fn db() -> PgPool{
    let pool = sqlx::postgres::PgPool::connect("postgres://postgres:PYPYster0312@localhost:5432/test_1").await.unwrap();
    pool
}

async fn get_todo_list(State(pool): State<PgPool>) -> impl IntoResponse{
    let todo : Vec<Todo> = sqlx::query_as("SELECT * FROM todo_list").fetch_all(&pool).await.unwrap();

    let json_todo = serde_json::to_string_pretty(&todo).unwrap();
    (StatusCode::OK,json_todo).into_response()
}

async fn get_single(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse{
    let todo:Vec<Todo> = sqlx::query_as("SELECT * FROM todo_list WHERE id = $1").bind(&id).fetch_all(&pool).await.unwrap();

    if todo.len() == 0 {
        (StatusCode::NOT_FOUND,"No Such Task".to_string()).into_response()
    } else {
        let todo_json = serde_json::to_string_pretty(&todo[0]).unwrap();
        (StatusCode::OK,todo_json).into_response()
    }
}

async fn add_todo(State(pool): State<PgPool>, Json(todo_req): Json<TodoRequest>) -> impl IntoResponse {
    sqlx::query("INSERT INTO todo_list (content) VALUES ($1)").bind(&todo_req.content).execute(&pool).await.unwrap();

    (StatusCode::OK, "Add New Task".to_string())
}

async fn delete_todo(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse{
    let todo:Vec<Todo> = sqlx::query_as("SELECT * FROM todo_list WHERE id = $1").bind(&id).fetch_all(&pool).await.unwrap();

    if todo.len() == 0 {
        (StatusCode::NOT_FOUND,"No Such Task".to_string()).into_response()
    } else {
        sqlx::query("DELETE FROM todo_list WHERE id = $1").bind(&id).execute(&pool).await.unwrap();
        (StatusCode::OK,"Delete Task".to_string()).into_response()
    }
}

async fn update_todo(State(pool): State<PgPool>, Path(id): Path<i32>, Json(todo_req): Json<TodoRequest>) -> impl IntoResponse{
    let todo:Vec<Todo> = sqlx::query_as("SELECT * FROM todo_list WHERE id = $1").bind(&id).fetch_all(&pool).await.unwrap();

    if todo.len() == 0 {
        (StatusCode::NOT_FOUND,"No Such Task".to_string()).into_response()
    } else {
        sqlx::query("UPDATE todo_list SET content = $1 WHERE id =$2").bind(&todo_req.content).bind(&id).execute(&pool).await.unwrap();
        (StatusCode::OK,"Update Task".to_string()).into_response()
    } 
}
#[derive(FromRow,Serialize)]
struct Todo {
    id: i32,
    content: String,
}

#[derive(Deserialize)]
struct TodoRequest {
    content: String,
}