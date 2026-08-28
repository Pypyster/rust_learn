use axum::{Router, extract::Path, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello/{greetin}/{name}", get(hello));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    let addr = listener.local_addr().unwrap();
    println!("Server is listening on {}", addr);

    if let Err(e) = open::that(format!("http://127.0.0.1:{}/hello/me", addr.port())) {
        eprintln!("Не удалось открыть браузер: {}", e);
    }

    axum::serve(listener, app).await.unwrap();
}

async fn hello(Path((greeting,name)):Path<(String, String)>) -> String {
    format!("{greeting} {name}")
}

