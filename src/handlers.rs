use crate::db;
use crate::views::{board_page, board_partial, boards_partial, index_page, posts_partial};
use maud::html;
use sqlx::{Pool, Postgres};
use std::collections::HashMap;
use std::convert::Infallible;
use warp::http::StatusCode;

pub async fn index(pool: Pool<Postgres>) -> Result<impl warp::Reply, Infallible> {
    let boards_partial = match db::boards_get(pool).await {
        Ok(boards) => boards_partial(boards),
        Err(_) => html! { p { "No boards found" } },
    };

    Ok(index_page(boards_partial))
}

pub async fn board(
    board_name: String,
    pool: Pool<Postgres>,
) -> Result<impl warp::Reply, Infallible> {
    let Ok(board) = db::board_get(pool.clone(), &board_name).await else {
        return Ok(html! { (StatusCode::NOT_FOUND) });
    };

    let board_partial = board_partial(&board);
    let threads_partial = match db::threads_get(pool, &board).await {
        Ok(threads) => posts_partial(threads),
        Err(_) => html! { p { "No threads found" } },
    };

    Ok(board_page(board_partial, threads_partial))
}

pub async fn thread(
    board_name: String,
    data: HashMap<String, String>,
    pool: Pool<Postgres>,
) -> Result<impl warp::Reply, Infallible> {
    let Ok(board) = db::board_get(pool.clone(), &board_name).await else {
        return Ok(html! { (StatusCode::NOT_FOUND) });
    };

    match db::thread_post(
        pool,
        &board,
        data.get("body").unwrap_or(&String::new()).to_string(),
    )
    .await
    {
        Ok(_id) => Ok(html! { meta http-equiv="refresh" content="0" {} }),
        Err(e) => {
            eprintln!("error while posting thread: {e}");
            Ok(html! { (StatusCode::SERVICE_UNAVAILABLE) })
        }
    }
}
