use axum::{Json, Router, extract::{Path, State}, response::IntoResponse, routing::{delete, get, put, post}};
use reqwest::StatusCode;
use sea_orm::{ActiveModelTrait, ActiveValue, ConnectionTrait, Database, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait, Statement};
use serde::Deserialize;
use thread_help::todo::{Entity as TodoEntity, ActiveModel as TodoActiveModel};

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

async fn app() -> Router {
    let conn = db().await;
    Router::new()
    .route("/", get(|| async {"Home Page"}))
    .route("/todo_list", get(get_todo_list))
    .route("/add_todo", post(add_todo))
    .route("/todo/{id}", get(get_single))
    .route("/delete_todo/{id}", delete(delete_todo))
    .route("/update_todo/{id}", put(update_todo))
    .with_state(conn)
}

async fn get_todo_list(State(conn): State<DatabaseConnection>) -> impl IntoResponse{
    let todos = TodoEntity::find().all(&conn).await.unwrap();

    let todos_json = serde_json::to_string_pretty(&todos).unwrap();
    (StatusCode::OK,todos_json)
}

async fn add_todo(State(conn): State<DatabaseConnection>, Json(todo_req): Json<TodoRequest>) -> impl IntoResponse{
    let todo = TodoActiveModel{
        content: ActiveValue::set(todo_req.content),
        ..Default::default()
    };
    todo.insert(&conn).await.unwrap();

    (StatusCode::OK, "Add New Task")
}

async fn get_single(State(conn): State<DatabaseConnection>,Path(id): Path<i32>) -> impl IntoResponse{
    let todos = TodoEntity::find_by_id(id).one(&conn).await.unwrap();

    match todos {
        Some(todo) => {
            let todo_json = serde_json::to_string_pretty(&todo).unwrap();
            (StatusCode::OK,todo_json)
        },
        None => {
            let msg = format!("Task ID: {id} Not Found");
            (StatusCode::NOT_FOUND,msg)
        }
    }
}

async fn delete_todo(State(conn): State<DatabaseConnection>,Path(id): Path<i32>) -> impl IntoResponse{
    let todos = TodoEntity::find_by_id(id).one(&conn).await.unwrap();

    match todos {
        Some(todo) => {
            todo.delete(&conn).await.unwrap();
            (StatusCode::OK,"Delete Task".to_string())
        },
        None => {
            let msg = format!("Task ID: {id} Not Found");
            (StatusCode::NOT_FOUND,msg)
        }
    }
}

async fn update_todo (State(conn): State<DatabaseConnection>,Path(id): Path<i32>, Json(todo_req): Json<TodoRequest>) -> impl IntoResponse{
    let todos = TodoEntity::find_by_id(id).one(&conn).await.unwrap();

    match todos {
        Some(todo) => {
            let mut todo = todo.into_active_model();
            todo.content = ActiveValue::set(todo_req.content);
            todo.update(&conn).await.unwrap();
            (StatusCode::OK,"Update Task".to_string())
        },
        None => {
            let msg = format!("Task ID: {id} Not Found");
            (StatusCode::NOT_FOUND,msg)
        }
    }
}

async fn db() -> DatabaseConnection {
    let conn = Database::connect("postgres://postgres:PYPYster0312@localhost:5432/test_1").await.unwrap();

    let db_backend = conn.get_database_backend();

    conn.execute_unprepared(
    "CREATE TABLE IF NOT EXISTS todos (
        id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
        content TEXT)").await.unwrap();
    conn
}

#[derive(Deserialize)]
struct TodoRequest {
    content: String,
}

