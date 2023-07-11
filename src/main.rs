use axum_assessment_custom_responses::run;

#[tokio::main]
async fn main() {
    match run().await {
        Ok(_) => println!("Server exited"),
        Err(error) => eprintln!("Server crashed with error: {}", error),
    }
}
