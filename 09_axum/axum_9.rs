use axum::Extension;
use axum::extract::Request;
use axum::http::HeaderValue;
use axum::middleware::{Next, from_fn, from_fn_with_state};
use axum::{Json, Router, extract::State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Method, StatusCode};
use serde::{Deserialize, Serialize};
use tokio_postgres::{Client, NoTls};
use tower_http::cors::{CorsLayer, Any};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::main]
async fn main() {
    let client = data_base().await;
    let app = app(client);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("You addr: {}", listener.local_addr().unwrap());
    
    // if let Err(e) = open::that("http://127.0.0.1:8080/") {
    //     eprintln!("Can not open browser: {}", e);
    // }
    
    axum::serve(listener, app).await.unwrap();
}

fn app(client: Client) -> Router{
    let cors_Layer = CorsLayer::new()
    .allow_methods(Any)
    .allow_headers([CONTENT_TYPE,AUTHORIZATION])
    .allow_origin("http://localhost:4000".parse::<HeaderValue>().unwrap());

    Router::new().route("/", get(|| async {"你好世界"}))
    .route("/user/signup",post(signup))
    .route("/user/signin", post(signin))
    .route("/protected", get(protected).layer(from_fn(auth)))
    .with_state(Arc::new(client))
}

async fn signup(State(client): State<Arc<Client>>, Json(user): Json<UserRequest>) -> impl IntoResponse{
    println!("{:?}", user);
    let has_pas = bcrypt::hash(user.password, 10).unwrap();
    println!("Hashed password: {has_pas}");
    client
    .execute(
        "INSERT INTO users (username, password_hash) VALUES ($1, $2)",
        &[&user.username, &has_pas],
         )
    .await
    .unwrap();
    (StatusCode::OK, "Sign Up successful!").into_response()
}

async fn signin(State(client): State<Arc<Client>>, Json(user): Json<UserRequest>) -> impl IntoResponse{
    let rows = client.query("SELECT * FROM users WHERE username = $1", &[&user.username])
    .await.unwrap();

    let hash_pas: String = rows[0].get(2);

    let is_vaid = bcrypt::verify(user.password, &hash_pas).unwrap();

    if is_vaid {
        let username: String = rows[0].get(1);
        let claims = Claims{
            sub: username,
            exp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + 60*60
        };
        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_bytes())).unwrap();
        (StatusCode::OK, token).into_response()
    } else {
        (StatusCode::UNAUTHORIZED, "Incorret password!").into_response()
    }
}

async fn protected(Extension(username): Extension<String>) -> impl IntoResponse{
    let res = format!("Hello {}", username);
    (StatusCode::OK,res).into_response()
}

async fn auth(State(state): State<Arc<Message>>, mut request: Request, next: Next) -> impl IntoResponse{
    println!("{:?}", state);
    match request.headers().get("authorization") {
        None => (StatusCode::UNAUTHORIZED, "No token!").into_response(),
        Some(header_value) => {
            let token = header_value.to_str().unwrap();
            match decode(token, &DecodingKey::from_secret("secret".as_bytes()), &Validation::default()) {
                Err(e) => (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
                Ok(token_data) => {
                    let claims: Claims = token_data.claims;
                    let username = claims.sub;
                    request.extensions_mut().insert(username);
                    let req = next.run(request).await;
                    req
                }
            }
        }
    }
}

async fn data_base() -> Client {
    let connection_str = "host=localhost port=5432 user=stevae password=987456 dbname=test_1";

    let (client,connection) = tokio_postgres::connect(&connection_str, NoTls).await.unwrap();
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("DB error: {}", e);
        }
    });

    client
}

#[derive(Debug,Deserialize)]
struct UserRequest {
    username: String,
    password: String,
}

#[derive(Debug,Deserialize,Serialize)]
struct  Claims{
    sub: String,
    exp: u64,
}

#[derive(Debug)]
struct Messsage{
    content: String,
}