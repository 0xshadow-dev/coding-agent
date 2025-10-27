use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let api_key = std::env::var("GEMINI_API_KEY")?;

    println!("Environment loaded");
    println!("API KEY FOUND");
    println!("Async runtime working");

    Ok(())
}
