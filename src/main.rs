use maud::{DOCTYPE, Markup, html};
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::prelude::FromRow;
use sqlx::{Error, Pool, Postgres, query_file};
use std::convert::Infallible;
use std::env;
use std::fmt::Display;
use warp::Filter;

#[derive(FromRow)]
struct BoardQuery {
    pub id: i64,
    pub name: String,
    pub description: String,
}

impl Display for BoardQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"
                id: {},
                name: {},
                description: {}
            "#,
            self.id, self.name, self.description
        )
    }
}

// This doesnt need to be in a function
// but I'd like to try since we'll probably
// need to put stuff like this in functions later
async fn get_boards(conn: &sqlx::Pool<sqlx::Postgres>) -> Result<Vec<BoardQuery>, Error> {
    let rows = sqlx::query_as!(BoardQuery, "SELECT * FROM boards ORDER BY name")
        .fetch_all(conn)
        .await?;
    Ok(rows)
}

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

fn boards_partial(boards: Result<Vec<BoardQuery>, Error>) -> Markup {
    match boards {
        Ok(boards) => {
            html! {
                @for board in boards {
                    a href="/b/g" { (board) }
                }
            }
        }
        Err(_) => {
            html! {
                p { "No boards found" }
            }
        }
    }
}

async fn index_page(boards: Markup) -> Result<Markup, Infallible> {
    Ok(html! {
        (page("brd"))
        p { "welcome to brd" }

        h2 { "boards" }
        (boards)

        h2 { "description" }
        p { "A simple imageboard site that supports private walled-garden communication" }
    })
}

fn with_pool(
    pool: Pool<Postgres>,
) -> impl Filter<Extract = (Pool<Postgres>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

// TODO: potentially put this in a handler module
async fn index(pool: Pool<Postgres>) -> Result<impl warp::Reply, Infallible> {
    index_page(boards_partial(get_boards(&pool).await)).await
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

    // Make a simple query to return the given parameter
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    assert_eq!(row.0, 150);

    let index_route = warp::path::end()
        .and(with_pool(pool.clone()))
        .and_then(index);

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
