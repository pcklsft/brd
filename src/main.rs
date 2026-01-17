use sqlx::postgres::PgPoolOptions;
use sqlx::query_file;
use std::env;

use brd::filters;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load env variables from .env
    dotenvy::dotenv()?;

    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL")?)
        .await?;

    query_file!("queries/seed_data.sql").execute(&pool).await?;

    let api = filters::api(pool);
    warp::serve(api).run(([127, 0, 0, 1], 8000)).await;

    Ok(())
}
