use reqwest::StatusCode;

pub async fn standard_with_text() -> (StatusCode, &'static str) {
    (StatusCode::ACCEPTED, "expected response")
}
