use crate::models::{Board, Post};
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

pub async fn threads_get(pool: Pool<Postgres>, board: &Board) -> Result<Vec<Post>, sqlx::Error> {
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

pub async fn thread_get(pool: Pool<Postgres>, thread_id: i64) -> Result<Vec<Post>, sqlx::Error> {
    sqlx::query_as!(
        Post,
        r#"
          SELECT * FROM posts
          WHERE (id = $1 AND parent IS NULL)
            OR (parent = $1)
          ORDER BY id
        "#,
        thread_id
    )
    .fetch_all(&pool)
    .await
}

// TODO: forbid empty posts
pub async fn post_create(
    pool: Pool<Postgres>,
    board: &Board,
    parent: Option<i64>,
    body: String,
    file_name: Option<String>,
) -> Result<i64, Box<dyn std::error::Error>> {
    let post_id = sqlx::query!(
        r#"
            INSERT INTO posts (body, parent, board_id, file_name)
            VALUES ($1, $2, $3, $4)
            RETURNING id;
        "#,
        body,
        parent,
        board.id,
        file_name,
    )
    .fetch_one(&pool)
    .await?
    .id;

    Ok(post_id)
}
