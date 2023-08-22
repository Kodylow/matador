use anyhow::Result;
use axum::{
    extract::{DefaultBodyLimit, Multipart},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Router,
};
use std::{convert::Infallible, net::SocketAddr};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // region:   --- Start Server
    tracing_subscriber::fmt::init();
    let router = Router::new()
        .route("/file", post(upload))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1024));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("Server listening on {addr}");
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
    // endregion: --- Start Server

    Ok(())
}

#[axum::debug_handler]
async fn upload(mut multipart: Multipart) -> Result<impl IntoResponse, Infallible> {
    while let Ok(Some(mut field)) = multipart.next_field().await {
        let name = field.name().unwrap().to_string();
        let mut data = Vec::new();
        while let Some(chunk) = field.chunk().await.unwrap() {
            data.extend_from_slice(&chunk);
        }
        println!("Length of `{}` is {} bytes", name, data.len());

        // Save the file
        if name == "image" {
            let path = format!("./local_directory/{}.png", name);
            tokio::fs::write(path, &data).await.unwrap();
            println!("File `{}` has been saved", name);
        }
    }

    Ok(StatusCode::OK)
}
