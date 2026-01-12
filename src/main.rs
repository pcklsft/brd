use sqlx::postgres::PgPoolOptions;
use sqlx::query_file;
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

    query_file!("queries/seed_data.sql").execute(&pool).await?;

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
        boards_get(pool.clone())
            .or(user_get())
            .or(board_get(pool))
            .or(static_assets())
    }

    pub fn boards_get(
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

    pub fn board_get(
        pool: Pool<Postgres>,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("b" / String)
            .and(with_pool(pool))
            .and_then(handlers::board)
    }

    pub fn static_assets()
    -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path("assets").and(warp::fs::dir("assets"))
    }
}

mod handlers {
    use super::db::{board_get, boards_get, threads_get};
    use super::views::{board_page, board_partial, boards_partial, index_page, posts_partial};
    use maud::html;
    use sqlx::{Pool, Postgres};
    use std::convert::Infallible;
    use warp::http::StatusCode;

    pub async fn index(pool: Pool<Postgres>) -> Result<impl warp::Reply, Infallible> {
        let boards_partial = match boards_get(pool).await {
            Ok(boards) => boards_partial(boards),
            Err(_) => html! { p { "No boards found" } },
        };

        Ok(index_page(boards_partial))
    }

    pub async fn board(
        board_name: String,
        pool: Pool<Postgres>,
    ) -> Result<impl warp::Reply, Infallible> {
        let board = match board_get(pool.clone(), &board_name).await {
            Ok(board) => board,
            Err(_) => {
                return Ok(html! { (StatusCode::NOT_FOUND) });
            }
        };

        let board_partial = board_partial(&board);

        let threads_partial = match threads_get(pool, &board).await {
            Ok(threads) => posts_partial(threads),
            Err(_) => html! { p { "No threads found" } },
        };

        Ok(board_page(board_partial, threads_partial))
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

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Post {
        pub id: i64,
        pub body: String,
        pub parent: Option<i64>,
        pub board_id: i64,
    }
}

mod views {
    use super::models::{Board, Post};
    use maud::{DOCTYPE, Markup, html};

    pub fn header(page_title: &str) -> Markup {
        html! {
            (DOCTYPE)
            meta charset="utf-8";
            title { (page_title) }
            link rel="stylesheet" href="/assets/css/style.css";
        }
    }

    pub fn page(title: &str) -> Markup {
        html! {
            (header(title))
            h1 { (title) }
        }
    }

    pub fn boards_partial(boards: Vec<Board>) -> Markup {
        html! {
                @for board in boards {
                    a href="/b/g" { (board.name) }
                }
        }
    }

    pub fn index_page(boards: Markup) -> Markup {
        html! {
            (page("brd"))
            p { "welcome to brd" }

            h2 { "boards" }
            (boards)

            h2 { "description" }
            p { "A simple imageboard site that supports private walled-garden communication" }
        }
    }

    pub fn board_partial(board: &Board) -> Markup {
        html! {
            (page(&board.name))
        }
    }

    pub fn posts_partial(posts: Vec<Post>) -> Markup {
        println!("{:#?}", posts);
        html! {
            @for post in posts {
                pre {
                    "id: " (post.id) "\n"
                    "body: " (post.body) "\n"
                }
            }
        }
    }

    pub fn board_page(board: Markup, posts: Markup) -> Markup {
        html! {
            (board)
            a href="/" { "<= Go back" }
            (posts)
        }
    }
}

mod db {
    use super::models::{Board, Post};
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

    pub async fn board_get(pool: Pool<Postgres>, board_name: &str) -> Result<Board, sqlx::Error> {
        sqlx::query_as!(Board, "SELECT * FROM boards WHERE name = $1", board_name)
            .fetch_one(&pool)
            .await
    }

    pub async fn threads_get(
        pool: Pool<Postgres>,
        board: &Board,
    ) -> Result<Vec<Post>, sqlx::Error> {
        sqlx::query_as!(
            Post,
            r#"
            SELECT * FROM posts
            WHERE board_id = $1 AND parent IS NULL
            ORDER BY id
            "#,
            board.id
        )
        .fetch_all(&pool)
        .await
    }
}
