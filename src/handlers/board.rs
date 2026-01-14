use std::convert::Infallible;

use maud::html;
use sqlx::{Pool, Postgres};
use warp::http::StatusCode;

use crate::{
    db,
    views::{board_page, threads_partial},
};

pub async fn get(board_name: String, pool: Pool<Postgres>) -> Result<impl warp::Reply, Infallible> {
    let Ok(board) = db::board_get(pool.clone(), &board_name).await else {
        return Ok(html! { (StatusCode::NOT_FOUND) });
    };

    let threads_partial = match db::threads_get(pool, &board).await {
        Ok(threads) => threads_partial(&board, threads),
        Err(_) => html! { p { "No threads found" } },
    };

    Ok(board_page(&board, threads_partial))
}
