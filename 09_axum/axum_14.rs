use axum::{Router, extract::{State, WebSocketUpgrade, ws::{Message, WebSocket}}, response::IntoResponse, routing::{Route, get}};
use futures::SinkExt;
use tokio::sync::broadcast::{Sender, channel};
use futures_util::StreamExt;

#[tokio::main]
async fn main() {
    let (tx, _) = channel(100);
    let app = app(tx);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    // if let Err(e) = open::that("ws://127.0.0.1:8080/ws") {
    //     eprintln!("Can not open browser: {}", e);
    // }

    axum::serve(listener, app).await.unwrap();
}
fn app(tx: Sender<String>) -> Router{
    Router::new().route("/ws", get(ws_handler)).with_state(tx)
}
async fn ws_handler(State(tx): State<Sender<String>>, ws: WebSocketUpgrade) -> impl IntoResponse{
    ws.on_upgrade(|socket| handler_ws(tx, socket))
}

async fn handler_ws(tx: Sender<String>, socket: WebSocket) {
    let (mut sender, mut receiver) = socket.split();

    let mut rx = tx.subscribe();

    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            sender.send(Message::from(msg)).await.unwrap();  
        }
    });

    while let Some(Ok(msg)) = receiver.next().await {
        match msg {
            Message::Text(content) => {
                tx.send(content.to_string()).unwrap();
            },
            _ => ()
        }
    }
}