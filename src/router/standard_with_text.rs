use reqwest::StatusCode;

/// Update this route handler to return a status code 202 ACCEPTED with the static string "expected response"
pub async fn standard_with_text() -> (StatusCode, &'static str) {
    (StatusCode::ACCEPTED, "expected response")
}
