use eyre::Result;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

const BASE_API_URL: &str = "http://localhost:3000";

#[tokio::test]
async fn api_returns_standard_code_with_text() -> Result<()> {
    let url = format!("{BASE_API_URL}/standard_with_text");
    let response = reqwest::get(url).await?;
    let expected_status = StatusCode::ACCEPTED;

    assert_eq!(response.status(), expected_status);

    let response_text = response.text().await?;
    let expected_text = "expected response";

    assert_eq!(response_text, expected_text);

    Ok(())
}

#[tokio::test]
async fn api_returns_non_standard_code_with_text() -> Result<()> {
    let url = format!("{BASE_API_URL}/non_standard_with_text");
    let response = reqwest::get(url).await?;
    let status_code = response.status();
    let expected_status_code = 599;

    assert_eq!(status_code, expected_status_code);

    let text = response.text().await?;
    let expected_text = "I'm a non-standard error";

    assert_eq!(text, expected_text);
    Ok(())
}

#[tokio::test]
async fn api_returns_non_standard_code_with_json() -> Result<()> {
    let url = format!("{BASE_API_URL}/non_standard_with_json");
    let response = reqwest::get(url).await?;
    let status_code = response.status();
    let expected_status_code = 588;

    assert_eq!(status_code, expected_status_code);

    let json_error = response.json::<JsonError>().await?;
    let expected_json_error = JsonError {
        code: 1,
        message: "There was an error in the server".to_owned(),
    };

    assert_eq!(json_error, expected_json_error);

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct JsonError {
    pub code: u16,
    pub message: String,
}
