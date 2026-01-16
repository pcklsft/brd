use crate::db;
use crate::views::{boards_partial, index_page};

use maud::html;
use sqlx::{Pool, Postgres};
use std::convert::Infallible;

pub mod board;
pub mod post;

pub async fn index(pool: Pool<Postgres>) -> Result<impl warp::Reply, Infallible> {
    let boards_partial = match db::board::all(pool).await {
        Ok(boards) => boards_partial(boards),
        Err(_) => html! { p { "No boards found" } },
    };

    Ok(index_page(boards_partial))
}
