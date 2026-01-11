use maud::{DOCTYPE, Markup, html};
use sqlx::postgres::PgPoolOptions;
use std::env;
use warp::Filter;

fn header(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        title { (page_title) }
        link rel="stylesheet" href="assets/css/style.css";
    }
}

fn page(title: &str) -> Markup {
    html! {
        (header(title))
        h1 { (title) }
    }
}

fn index_page() -> Markup {
    html! {
        (page("brd"))
        p { "welcome to brd" }

        h2 { "boards" }
        a href="/b/g" { "g" }

        h2 { "description" }
        p { "A simple imageboard site that supports private walled-garden communication" }
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

    let index = warp::path::end().map(|| index_page());

    let board_page = warp::path("b")
        .and(warp::path::param())
        .map(|param: String| page(&param));

    let user_page = warp::path("u")
        .and(warp::path::param())
        .map(|param: String| page(&format!("user {}", param)));

    #[rustfmt::skip]
    let static_assets = warp::path("assets")
        .and(warp::fs::dir("assets"));

    #[rustfmt::skip]
    let routes = warp::get().and(
        index
            .or(board_page)
            .or(user_page)
            .or(static_assets)
    );

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;

    Ok(())
}
