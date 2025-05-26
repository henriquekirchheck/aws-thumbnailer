use axum::{
    Router,
    http::{StatusCode, Uri, header},
    response::{IntoResponse, Response},
    routing::get,
};
use handlers::{handle_index, handle_uploads};
use lambda_http::{Error, run, tracing};
use rust_embed::Embed;
use std::env::set_var;

mod extensions;
mod handlers;
mod renderables;
mod views;

#[tokio::main]
async fn main() -> Result<(), Error> {
    unsafe { set_var("AWS_LAMBDA_HTTP_IGNORE_STAGE_IN_PATH", "true") };

    tracing::init_default_subscriber();

    let app = Router::new()
        .route("/", get(handle_index))
        .route("/uploads", get(handle_uploads))
        .route("/assets/{*file}", get(asset_handler));

    run(app).await
}

#[derive(Debug, Embed)]
#[folder = "$CARGO_MANIFEST_DIR/assets/"]
struct Assets;

async fn asset_handler(uri: Uri) -> Response {
    let path = uri.path().trim_start_matches("/assets/");
    match Assets::get(path) {
        Some(file) => (
            [(header::CONTENT_TYPE, file.metadata.mimetype())],
            file.data,
        )
            .into_response(),
        None => (
            StatusCode::NOT_FOUND,
            format!("404 Not Found {}", uri.path()),
        )
            .into_response(),
    }
}
