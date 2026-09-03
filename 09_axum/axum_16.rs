use serde::{Deserialize, Serialize};
use sqlx::{Executor, Pool, Sqlite, prelude::FromRow, sqlite};
use axum::{
    Json, Router,
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post, delete, put},
};
use reqwest::StatusCode;

#[tokio::main]
async fn main() {
    let pool = db().await;
    let app = app(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    let addr = listener.local_addr().unwrap();
    println!("Server is listening on {}", addr);

    if let Err(e) = open::that(format!("http://127.0.0.1:{}/", addr.port())) {
        eprintln!("Can not open browser: {}", e);
    }
    
    axum::serve(listener, app).await.unwrap();
}

async fn db() -> Pool<Sqlite> {
    let opt = sqlite::SqliteConnectOptions::new().filename("test.db").create_if_missing(true);

    let pool = sqlite::SqlitePool::connect_with(opt).await.unwrap();

    pool.execute("CREATE TABLE IF NOT EXISTS users_api (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT
    );").await.unwrap();

    pool

}

fn app(pool: Pool<Sqlite> ) -> Router {
    Router::new().route("/", get(|| async { "Home page"}))
    .route("/list", get(get_list))
    .route("/persone/{id}", get(get_singl))
    .route("/add_persone", post(add_new_persone))
    .route("/remove_persone/{id}", delete(remove_persone))
    .route("/update_persone/{id}", put(update_persone))
    .with_state(pool)
}

async fn get_list(State(pool): State<Pool<Sqlite>>) -> impl IntoResponse {
    let rows: Vec<User> = sqlx::query_as("SELECT * FROM users_api").fetch_all(&pool).await.unwrap();

    let json_data = serde_json::to_string_pretty(&rows).unwrap();
    (StatusCode::OK,json_data).into_response()
}

async fn add_new_persone( State(pool): State<Pool<Sqlite>>, Json(user_request): Json<UserRequest>) -> impl IntoResponse{
    sqlx::query("INSERT INTO users_api (name) VALUES ($1)").bind(&user_request.name).execute(&pool).await.unwrap();

    (StatusCode::OK, "Add new Persone".to_string()).into_response()
}

async fn get_singl( State(pool): State<Pool<Sqlite>>, Path(id): Path<i32>) -> impl IntoResponse{
    let rows:Vec<User> = sqlx::query_as("SELECT * FROM users_api WHERE id =$1").bind(&id).fetch_all(&pool).await.unwrap();
    
    if rows.len() == 0 {
        let msg = format!("Persone id: {} Not Found", id);

        (StatusCode::NOT_FOUND, msg).into_response()
    } else  {
        let persone = &rows[0];
        let json_data = serde_json::to_string_pretty(&persone).unwrap();
        (StatusCode::OK, json_data).into_response()
    }
}

async fn remove_persone( State(pool): State<Pool<Sqlite>>, Path(id): Path<i32>) -> impl IntoResponse{
    let rows:Vec<User> = sqlx::query_as("SELECT * FROM users_api WHERE id =$1").bind(&id).fetch_all(&pool).await.unwrap();
    
    if rows.len() == 0 {
        let msg = format!("Persone id: {} Not Found", id);

        (StatusCode::NOT_FOUND, msg).into_response()
    } else  {
        sqlx::query("DELETE FROM users_api WHERE id = $1").bind(&id).execute(&pool).await.unwrap();
        let msg = format!("Persone id: {} removed", id);
        (StatusCode::OK,msg).into_response()
    }
}

async fn update_persone( State(pool): State<Pool<Sqlite>>, Path(id): Path<i32>, Json(user_request): Json<UserRequest>) -> impl IntoResponse{
    let rows:Vec<User> = sqlx::query_as("SELECT * FROM users_api WHERE id =$1").bind(&id).fetch_all(&pool).await.unwrap();
    
    if rows.len() == 0 {
        let msg = format!("Persone id: {} Not Found", id);

        (StatusCode::NOT_FOUND, msg).into_response()
    } else  {
        sqlx::query("UPDATE users_api SET name = $1 WHERE id = $2").bind(&user_request.name).bind(&id).execute(&pool).await.unwrap();
        let msg = format!("Persone id: {} updated", id);
        (StatusCode::OK,msg).into_response()
    }
}

#[derive(FromRow, Serialize)]
struct User {
    id: i32,
    name: String,
}

#[derive(Deserialize)]
struct UserRequest{
    name: String,
}