use axum::{Router, response::IntoResponse, routing::get};
use axum::http::StatusCode;
use axum_session::{Key, Session, SessionConfig, SessionLayer, SessionStore};
use axum_session_sqlx::SessionSqlitePool;
use sqlx::{Pool, Sqlite};

#[tokio::main]
async fn main() {
    let pool = db().await;
    let session_store = session(pool).await;
    let app = app(session_store);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

fn app(session_store: SessionStore<SessionSqlitePool>) -> Router {
    Router::new()
        .route("/", get(|| async { "Home page" }))
        .route("/hello", get(hello))
        .route("/world", get(world))
        .layer(SessionLayer::new(session_store))
}

async fn hello(session: Session<SessionSqlitePool>) -> impl IntoResponse {
    let msg: String = session
        .get("Message")
        .unwrap_or_else(|| "No message set yet".to_string());
    (StatusCode::OK, msg)
}

async fn world(session: Session<SessionSqlitePool>) -> impl IntoResponse {
    session.set("Message", "Aloha!");
    (StatusCode::OK, "Set session").into_response()
}

async fn db() -> Pool<Sqlite> {
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
    use std::str::FromStr;

    let options = SqliteConnectOptions::from_str("sqlite://db.sqlite")
        .unwrap()
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .connect_with(options)
        .await
        .expect("Failed to connect to SQLite database");

    println!("Connected to db.sqlite at: {:?}", std::env::current_dir().unwrap());

    pool
}
async fn session(pool: Pool<Sqlite>) -> SessionStore<SessionSqlitePool> {
    let config = SessionConfig::default()
        .with_table_name("session_table")
        .with_key(Key::generate());

    SessionStore::<SessionSqlitePool>::new(Some(pool.clone().into()), config)
        .await
        .unwrap()
}