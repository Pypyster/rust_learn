use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema, http::GraphiQLSource};
use async_graphql_axum::GraphQL;
use axum::response::{Html, IntoResponse};
use axum::Router;
use axum::routing::get;

#[tokio::main]
async fn main() {
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

    let router = Router::new().route("/graphql", get(graphql).post_service(GraphQL::new(schema)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    if let Err(e) = open::that("http://127.0.0.1:8080/graphql") {
        eprintln!("Can not open browser: {}", e);
    }
    axum::serve(listener, router).await.unwrap();
}

async fn graphql() -> impl IntoResponse{
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}
struct Query;

#[Object]
impl Query {
    async fn hello(&self) -> String {
        "Hello".to_string()
    }
}