use contentful_migrations_rs::contentful_migration;

use contentful_migration::*;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let cma_token = env::var("CONTENTFUL_CMA_TOKEN").unwrap();
    let t = client::new(&cma_token);
    t.space("35m8orkiglvo")
        .environment("master")
        .send_req()
        .await?;

    Ok(())
}
