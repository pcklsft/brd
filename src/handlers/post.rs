use crate::{db, models::Board, views::thread_page};

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

pub async fn process_form(
    board: &Board,
    pool: Pool<Postgres>,
    data: FormData,
    accepted_text: Vec<String>,
    accepted_files: Vec<String>,
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    // Begin transaction
    let mut tx = pool.begin().await?;

    let fields: HashMap<String, String> = data
        .and_then(|mut field| async move {
            let mut bytes: Vec<u8> = Vec::new();

            // TODO: CHECK IF THIS IS A FIELD WE'RE ACCEPTING
            match field.filename() {
                Some(filename) => {
                    // Create based on file id
                    // TODO: REMOVE UNWRAP
                    let file = tokio::fs::OpenOptions::new()
                        .write(true)
                        .create(true)
                        .open(format!(
                            "assets/user_content/board/{}/{}/{}",
                            board.name, file_id, filename
                        ))
                        .await
                        .unwrap();

                    Ok((field.name().to_string(), file_id.to_string()))
                }
                None => {
                    // field.data() only returns a piece of the content, so we should call it over and over until it's complete
                    while let Some(content) = field.data().await {
                        let content = content.unwrap();
                        bytes.put(content);
                    }

                    Ok((
                        field.name().to_string(),
                        String::from_utf8_lossy(&*bytes).to_string(),
                    ))
                }
            }
        })
        .try_collect()
        .await?;

    tx.commit().await?;
    Ok(fields)
}

// TODO: IF parent, require an image to be attached
// always expect an image OR a body
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

    // TODO: inspect this more...
    // Get text fields
    // maybe we should have a model for the specific form
    let file_id;
    let fields: HashMap<String, Vec<u8>> = data
        .and_then(|mut field| async move {
            let mut bytes: Vec<u8> = Vec::new();

            if field.name() == "file" {}

            // field.data() only returns a piece of the content, so we should call it over and over until it's complete
            while let Some(content) = field.data().await {
                let content = content.unwrap();
                bytes.put(content);
            }

            Ok((field.name().to_string(), bytes))
        })
        .try_collect()
        .await
        .unwrap();

    let Some(file) = fields.get("file") else {
        // Couldn't get file
        todo!();
    };

    let Some(body) = fields.get("body") else {
        // Couldn't get body
        todo!();
    };

    let body = String::from_utf8_lossy(&*body).to_string();

    if let Ok(id) = db::post_create(pool, &board, parent, body, file).await {
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
