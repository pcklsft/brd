use crate::db::{board_get, boards_get, threads_get};
use crate::views::{board_page, board_partial, boards_partial, index_page, posts_partial};
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
