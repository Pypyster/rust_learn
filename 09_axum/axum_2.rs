use axum::{Router, extract::Path, routing::{delete, get, patch, post,put}};

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
    Router::new().route("/hello", get(|| async {"get"})
    .post(|| async {"post"})
    .put(|| async {"put"})
    .patch(|| async {"patch"})
    .delete(|| async {"delete"}))
}

