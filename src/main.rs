use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let api_key = std::env::var("GEMINI_API_KEY")?;

    println!("Environment loaded");
    println!("✓ API key found: {}...", &api_key[..20]);
    println!("Async runtime working");

    Ok(())
}
