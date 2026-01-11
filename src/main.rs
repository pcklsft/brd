use maud::{DOCTYPE, Markup, html};
use sqlx::postgres::PgPoolOptions;
use std::env;
use warp::Filter;

fn header(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        title { (page_title) }
    }
}

fn page(title: &str) -> Markup {
    html! {
        (header(title))
        h1 { (title) }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load env variables from .env
    dotenvy::dotenv()?;

    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL")?)
        .await?;

    // Make a simple query to return the given parameter
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    assert_eq!(row.0, 150);

    let index_page = warp::any().map(|| page("brd"));
    warp::serve(index_page).run(([127, 0, 0, 1], 8000)).await;

    Ok(())
}
