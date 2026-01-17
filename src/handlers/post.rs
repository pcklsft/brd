use crate::{db, models::Board, views::thread_page};

use std::{collections::HashMap, convert::Infallible, path::Path};

use bytes::{Buf, BufMut};
use futures_util::{StreamExt, TryStreamExt};
use maud::html;
use sqlx::{Pool, Postgres};
use tokio::{
    fs::{self, File},
    io::{AsyncWriteExt, BufWriter},
};
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

    let fields: HashMap<String, Vec<u8>> = data
        .filter_map(|field| async move {
            if let Ok(ref field) = field
                && let Some(filename) = field.filename()
                && filename.is_empty()
            {
                None
            } else {
                Some(field)
            }
        })
        .and_then(|mut field| {
            let board_name = board.name.clone();

            async move {
                if field.name() == "file"
                    && let Some(filename) = field.filename()
                {
                    let path = format!(
                        "assets/user_content/board/{board_name}/{}/{filename}",
                        0, // TODO: get file id
                    );

                    let prefix = Path::new(&path).parent().unwrap();
                    fs::create_dir_all(prefix).await.unwrap();

                    let file = fs::OpenOptions::new()
                        .write(true)
                        .create(true)
                        .open(path.clone())
                        .await;

                    let file = match file {
                        Ok(file) => file,
                        Err(e) => panic!("{e:#?}"),
                    };

                    let mut writer = BufWriter::new(file);

                    while let Some(content) = field.data().await {
                        let content = content.unwrap();
                        writer.write_all(content.chunk()).await.unwrap();
                    }

                    let Ok(()) = writer.flush().await else {
                        todo!();
                    };

                    Ok((field.name().to_string(), path.into()))
                } else {
                    let mut bytes: Vec<u8> = Vec::new();

                    while let Some(content) = field.data().await {
                        let content = content.unwrap();
                        bytes.put(content);
                    }

                    Ok((field.name().to_string(), bytes))
                }
            }
        })
        .try_collect()
        .await
        .unwrap();

    let Some(body) = fields.get("body") else {
        // Couldn't get body
        todo!();
    };

    let body = String::from_utf8_lossy(&*body).to_string();

    match db::post_create(
        pool,
        &board,
        parent,
        body,
        fields
            .get("file")
            .map(|bytes| String::from_utf8_lossy(&*bytes).to_string()),
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
        Err(e) => panic!("{e:#?}"),
    }
}
