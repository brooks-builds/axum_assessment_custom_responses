mod non_standard_error_with_text;
mod standard_with_text;

use axum::{routing::get, Router};
use non_standard_error_with_text::non_standard_error_with_text;
use standard_with_text::standard_with_text;

pub async fn create_router() -> Router {
    Router::new()
        .route("/standard_with_text", get(standard_with_text))
        .route("/non_standard_with_text", get(non_standard_error_with_text))
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
