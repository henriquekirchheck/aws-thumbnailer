use axum::response::IntoResponse;
use axum_htmx::HxRequest;
use hypertext::prelude::*;

use crate::views::{document::Document, index::index, nav::Nav, uploads::uploads};

fn maybe_document<'a, R: Renderable>(
    HxRequest(hx_request): HxRequest,
    current: &'a str,
    children: R,
) -> impl IntoResponse {
    maud! {
        @if hx_request {
            Nav current=(current) oob=(true);
            (children)
        } @else {
            Document current=(current) {
                (children)
            }
        }
    }
}

pub async fn handle_index(hx_request: HxRequest) -> impl IntoResponse {
    maybe_document(hx_request, "/", index())
}

pub async fn handle_uploads(hx_request: HxRequest) -> impl IntoResponse {
    maybe_document(hx_request, "/uploads", uploads())
}
