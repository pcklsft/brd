use maud::{DOCTYPE, Markup, html};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, query_file};
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

// This doesnt need to be in a function
// but I'd like to try since we'll probably
// need to put stuff like this in functions later
async fn get_boards(conn: &sqlx::Pool<sqlx::Postgres>) -> Result<(), Error> {
    let rows = sqlx::query("SELECT * FROM boards ORDER BY name")
        .fetch_all(conn)
        .await?;

    println!("{rows:#?}");

    Ok(())
}

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

    get_boards(&pool).await?;

    // Make a simple query to return the given parameter
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    assert_eq!(row.0, 150);

    let index_route = warp::path::end().map(|| index_page());

    let board_route = warp::path("b")
        .and(warp::path::param())
        .map(|param: String| page(&param));

    let user_route = warp::path("u")
        .and(warp::path::param())
        .map(|param: String| page(&format!("user {}", param)));

    #[rustfmt::skip]
    let static_assets_route = warp::path("assets")
        .and(warp::fs::dir("assets"));

    #[rustfmt::skip]
    let routes = warp::get().and(
        index_route
            .or(board_route)
            .or(user_route)
            .or(static_assets_route)
    );

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;

    Ok(())
}
