use sqlx::{Pool, Postgres};

use crate::models::{Board, Post};

pub async fn all(pool: Pool<Postgres>, board: &Board) -> Result<Vec<Post>, sqlx::Error> {
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

pub async fn thread(pool: Pool<Postgres>, thread_id: i64) -> Result<Vec<Post>, sqlx::Error> {
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

pub async fn create(
    pool: Pool<Postgres>,
    board: &Board,
    parent: Option<i64>,
    body: String,
) -> Result<i64, sqlx::Error> {
    sqlx::query!(
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
    .await
    .map(|post| post.id)
}
