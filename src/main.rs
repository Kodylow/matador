pub use self::error::{Error, Result};

use axum::{
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{self, get_service},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;

mod ctx;
mod error;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the ModelController
    let mc = model::ModelController::new().await?;

    let routes_apis = web::routes_tickets::routes(mc.clone()).route_layer(middleware::from_fn(
        web::middleware_auth::middleware_require_auth,
    ));

    // region:   --- Start Server
    let router = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        // layers are executed in reverse order, from bottom to top
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::middleware_auth::middleware_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}");
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
    // endregion: --- Start Server

    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:12} - main_response_mapper", "RESPONSE_MAPPER");
    let uuid = Uuid::new_v4();

    // -- Get the eventual response error
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    // -- If client error, build new response
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string()
                }
            });

            println!("--> client_error_body: {client_error_body}");

            // Build the new response from the client_error_body
            // when you deref it impls Copy so you can still have it araound for later that's cool
            (*status_code, Json(client_error_body)).into_response()
        });
    println!("--> server log line - {uuid} - Error: {service_error:?}");
    println!();
    res
}

// region:      -- Routes Static
fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
// endregion:   -- Routes Static

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
