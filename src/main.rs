pub use self::error::{Error, Result};

use axum::Router;
use std::net::SocketAddr;

mod ctx;
mod error;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    // region:   --- Start Server
    let router = Router::new()
        // .merge(routes_hello())
        .nest("/openai", web::routes_openai::routes());
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}");
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
    // endregion: --- Start Server

    Ok(())
}

// async fn main_response_mapper(res: Response) -> Response {
//     println!("->> {:12} - main_response_mapper", "RESPONSE_MAPPER");
//     let uuid = Uuid::new_v4();

//     // -- Get the eventual response error
//     let service_error = res.extensions().get::<Error>();
//     let client_status_error = service_error.map(|se| se.client_status_and_error());

//     // -- If client error, build new response
//     let error_response = client_status_error
//         .as_ref()
//         .map(|(status_code, client_error)| {
//             let client_error_body = json!({
//                 "error": {
//                     "type": client_error.as_ref(),
//                     "req_uuid": uuid.to_string()
//                 }
//             });

//             println!("--> client_error_body: {client_error_body}");

//             // Build the new response from the client_error_body
//             // when you deref it impls Copy so you can still have it araound for later that's cool
//             (*status_code, Json(client_error_body)).into_response()
//         });
//     println!("--> server log line - {uuid} - Error: {service_error:?}");
//     println!();
//     res
// }

// // region:      -- Routes Static
// fn routes_static() -> Router {
//     Router::new().nest_service("/", get_service(ServeDir::new("./")))
// }
// // endregion:   -- Routes Static

// // region:      -- Routes Hello
// fn routes_hello() -> Router {
//     Router::new()
//         .route("/hello", routing::get(handler_hello))
//         .route("/hello/:name", routing::get(handler_hello2))
// }

// #[derive(Debug, Deserialize)]
// struct HelloParams {
//     name: Option<String>,
// }

// // example extraction from query param
// async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
//     println!("->> {:12} - handler_hello - {params:?}", "HANDLER");

//     let name = params.name.unwrap_or_else(|| "Anonymous".to_string());

//     Html(format!("Hello, <strong>{name}</strong>!"))
// }

// async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
//     println!("->> {:12} - handler_hello2 - {name:?}", "HANDLER");

//     Html(format!("Hello, <strong>{name}</strong>!"))
// }
// // endregion:   -- Handler Hello
