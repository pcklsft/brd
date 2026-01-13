use crate::db::{board_get, boards_get, thread_get, threads_get};
use crate::views::{board_page, boards_partial, index_page, thread_page, threads_partial};
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

    let threads_partial = match threads_get(pool, &board).await {
        Ok(threads) => threads_partial(&board, threads),
        Err(_) => html! { p { "No threads found" } },
    };

    Ok(board_page(&board, threads_partial))
}

pub async fn thread(
    board_name: String,
    thread_id: i64,
    pool: Pool<Postgres>,
) -> Result<impl warp::Reply, Infallible> {
    let board = match board_get(pool.clone(), &board_name).await {
        Ok(board) => board,
        Err(_) => {
            return Ok(html! { (StatusCode::NOT_FOUND) });
        }
    };

    let thread = match thread_get(pool, thread_id).await {
        Ok(thread) => thread,
        Err(_) => return Ok(html! { (StatusCode::NOT_FOUND )}),
    };

    Ok(thread_page(&board, thread))
}
