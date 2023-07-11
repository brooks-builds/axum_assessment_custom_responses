mod standard_with_text;

use axum::{routing::get, Router};
use standard_with_text::standard_with_text;

pub async fn create_router() -> Router {
    Router::new()
        .route("/standard_with_text", get(standard_with_text))
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
