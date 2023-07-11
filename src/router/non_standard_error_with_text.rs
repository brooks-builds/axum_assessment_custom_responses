use axum::http::StatusCode;

/// Change this route handler to return the non-standard error status code of 599 with the text "I'm a non-standard error"
pub async fn non_standard_error_with_text() -> Result<(StatusCode, &'static str), StatusCode> {
    let message = "I'm a non-standard error";

    Ok((
        StatusCode::from_u16(599).map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?,
        message,
    ))
}
