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
    file: Option<Vec<u8>>,
    file_name: Option<String>,
    file_type: Option<String>,
) -> Result<i64, Box<dyn std::error::Error>> {
    // If no body AND no file, then return an error
    if body == "" && (file.is_none() || file_name.is_none() || file_type.is_none()) {
        return Err("".into());
    }

    let mut tx = pool.begin().await?;

    let post_id = sqlx::query!(
        r#"
            INSERT INTO posts (body, parent, board_id)
            VALUES ($1, $2, $3)
            RETURNING id;
            "#,
        body,
        parent,
        board.id
    )
    .fetch_one(&pool)
    .await?
    .id;

    // Create and attach the file
    if let Some(file) = file
        && let Some(file_name) = file_name
        && let Some(file_type) = file_type
    {
        let file_id = sqlx::query!(
            r#"INSERT INTO files (file_name, file_type, board_id, post_id) VALUES ($1, $2, $3, $4) RETURNING id"#,
            file_name,
            file_type,
            board.id,
            post_id
        ).fetch_one(&pool).await?.id;

        // Apply file id to created post
        sqlx::query!(
            r#"UPDATE posts SET file_id = $1 WHERE id = $2"#,
            file_id,
            post_id
        )
        .fetch_one(&pool)
        .await?;

        // TODO: stream instead
        tokio::fs::write(
            format!(
                "assets/user_content/{}/{}/{}",
                board.name, post_id, file_name
            ),
            file,
        )
        .await?;
    }

    // If the function returns early at any point, then rollback the changes
    tx.commit().await;

    Ok(post_id)
}
