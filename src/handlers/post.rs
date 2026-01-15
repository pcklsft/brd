use crate::{db, views::thread_page};

use std::{collections::HashMap, convert::Infallible};

use maud::html;
use sqlx::{Pool, Postgres};
use warp::http::StatusCode;

pub async fn get(
    board_name: String,
    thread_id: i64,
    pool: Pool<Postgres>,
) -> Result<impl warp::Reply, Infallible> {
    let board = match db::board_get(pool.clone(), &board_name).await {
        Ok(board) => board,
        Err(_) => {
            return Ok(html! { (StatusCode::NOT_FOUND) });
        }
    };

    let thread = match db::thread_get(pool, thread_id).await {
        Ok(thread) => thread,
        Err(_) => return Ok(html! { (StatusCode::NOT_FOUND )}),
    };

    Ok(thread_page(&board, thread))
}

pub async fn create(
    board_name: String,
    parent: Option<i64>,
    data: HashMap<String, String>,
    pool: Pool<Postgres>,
) -> Result<impl warp::Reply, Infallible> {
    let Ok(board) = db::board_get(pool.clone(), &board_name).await else {
        todo!();
    };

    match db::post_create(
        pool,
        &board,
        parent,
        data.get("body").unwrap_or(&String::new()).to_string(),
    )
    .await
    {
        Ok(id) => {
            let path = format!("/b/{}/{}", board.name, parent.unwrap_or(id));
            let uri = warp::http::Uri::builder()
                .path_and_query(path)
                .build()
                .unwrap();
            Ok(warp::redirect(uri))
        }
        Err(_e) => todo!(),
    }
}
