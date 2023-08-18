use std::net::SocketAddr;

use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::{self, get_service},
    Router,
};
use serde::Deserialize;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // region:   --- Start Server
    let router = Router::new()
        .merge(routes_hello())
        .fallback_service(routes_static());
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}");
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
    // endregion: --- Start Server
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./static")))
}

// region:      -- Routes Hello
fn routes_hello() -> Router {
    Router::new()
        .route("/hello", routing::get(handler_hello))
        .route("/hello/:name", routing::get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// example extraction from query param
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.unwrap_or_else(|| "Anonymous".to_string());

    Html(format!("Hello, <strong>{name}</strong>!"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:12} - handler_hello2 - {name:?}", "HANDLER");

    Html(format!("Hello, <strong>{name}</strong>!"))
}
// endregion:   -- Handler Hello
