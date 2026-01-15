use crate::{db, views::thread_page};

use std::{collections::HashMap, convert::Infallible};

use bytes::BufMut;
use futures_util::TryStreamExt;
use maud::html;
use sqlx::{Pool, Postgres};
use warp::{filters::multipart::FormData, http::StatusCode};

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
    data: FormData,
    pool: Pool<Postgres>,
) -> Result<impl warp::Reply, Infallible> {
    let Ok(board) = db::board_get(pool.clone(), &board_name).await else {
        // No board found
        todo!();
    };

    let fields: HashMap<String, String> = data
        .and_then(|mut field| async move {
            let mut bytes: Vec<u8> = Vec::new();

            // field.data() only returns a piece of the content, so we should call it over and over until it's complete
            while let Some(content) = field.data().await {
                let content = content.unwrap();
                bytes.put(content);
            }
            Ok((
                field.name().to_string(),
                String::from_utf8_lossy(&*bytes).to_string(),
            ))
        })
        .try_collect()
        .await
        .unwrap();

    // TODO: Do not allow empty posts
    if let Ok(id) = db::post_create(
        pool,
        &board,
        parent,
        fields.get("body").unwrap_or(&String::new()).to_string(),
    )
    .await
    {
        let path = format!("/b/{}/{}", board.name, parent.unwrap_or(id));
        let uri = warp::http::Uri::builder()
            .path_and_query(path)
            .build()
            .unwrap();
        Ok(warp::redirect(uri))
    } else {
        // Post not successfully created
        todo!()
    }
}
