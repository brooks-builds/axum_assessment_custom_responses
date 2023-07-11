use eyre::Result;
use reqwest::StatusCode;

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
