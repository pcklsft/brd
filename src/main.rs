use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load env variables from .env
    dotenvy::dotenv()?;

    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL")?)
        .await?;

    let api = filters::api(pool);

    warp::serve(api).run(([127, 0, 0, 1], 8000)).await;

    Ok(())
}

mod filters {
    use super::db::with_pool;
    use super::handlers;
    use super::views::page;
    use sqlx::{Pool, Postgres};
    use warp::Filter;

    // All the routes combined
    pub fn api(
        pool: Pool<Postgres>,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        boards_list(pool)
            .or(user_get())
            .or(board_get())
            .or(static_assets())
    }

    pub fn boards_list(
        pool: Pool<Postgres>,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path::end()
            .and(warp::get())
            .and(with_pool(pool))
            .and_then(handlers::index)
    }

    pub fn user_get() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
    {
        warp::path("u")
            .and(warp::path::param())
            .map(|param: String| page(&format!("user {}", param)))
    }

    pub fn board_get() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
    {
        warp::path("b")
            .and(warp::path::param())
            .map(|param: String| page(&param))
    }

    pub fn static_assets()
    -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path("assets").and(warp::fs::dir("assets"))
    }
}

mod handlers {
    use super::db::boards_get;
    use super::views::{boards_partial, index_page};
    use sqlx::{Pool, Postgres};
    use std::convert::Infallible;

    pub async fn index(pool: Pool<Postgres>) -> Result<impl warp::Reply, Infallible> {
        index_page(boards_partial(boards_get(pool).await))
    }
}

mod models {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Board {
        pub id: i64,
        pub name: String,
        pub description: String,
    }
}

mod views {
    use super::models::Board;
    use maud::{DOCTYPE, Markup, html};
    use std::convert::Infallible;

    pub fn header(page_title: &str) -> Markup {
        html! {
            (DOCTYPE)
            meta charset="utf-8";
            title { (page_title) }
            link rel="stylesheet" href="assets/css/style.css";
        }
    }

    pub fn page(title: &str) -> Markup {
        html! {
            (header(title))
            h1 { (title) }
        }
    }

    pub fn boards_partial(boards: Result<Vec<Board>, sqlx::Error>) -> Markup {
        match boards {
            Ok(boards) => {
                html! {
                    @for board in boards {
                        a href="/b/g" { (board.name) }
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

    pub fn index_page(boards: Markup) -> Result<Markup, Infallible> {
        Ok(html! {
            (page("brd"))
            p { "welcome to brd" }

            h2 { "boards" }
            (boards)

            h2 { "description" }
            p { "A simple imageboard site that supports private walled-garden communication" }
        })
    }
}

mod db {
    use super::models::Board;
    use sqlx::{Pool, Postgres};
    use std::convert::Infallible;
    use warp::Filter;

    pub fn with_pool(
        pool: Pool<Postgres>,
    ) -> impl Filter<Extract = (Pool<Postgres>,), Error = Infallible> + Clone {
        warp::any().map(move || pool.clone())
    }

    pub async fn boards_get(pool: Pool<Postgres>) -> Result<Vec<Board>, sqlx::Error> {
        sqlx::query_as!(Board, "SELECT * FROM boards ORDER BY name")
            .fetch_all(&pool)
            .await
    }
}
