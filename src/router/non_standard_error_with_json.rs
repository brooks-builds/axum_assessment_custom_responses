use axum::{http::StatusCode, Json};

use crate::json_error::JsonError;

/// Change this route handler to return a custom error status code of 588 and a JSON object that looks like
///
/// ```json
/// {
///    "code": 1,
///    "message": "There was an error in the server"
/// }
/// ```
pub async fn non_standard_error_with_json() -> Result<(StatusCode, Json<JsonError>), StatusCode> {
    tracing::debug!("running non-standard error with json route handler");
    let json_error = JsonError::new()
        .code(1)
        .message("There was an error in the server")
        .build();
    let status_code = StatusCode::from_u16(588).map_err(|error| {
        tracing::debug!("error creating custom error: {:?}", error);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((status_code, Json(json_error)))
}
