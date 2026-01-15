use sqlx::{Pool, Postgres};
use warp::Filter;

use crate::{db, handlers};

pub mod board;
pub mod post;
pub mod user;

// All the routes combined
pub fn api(
    pool: Pool<Postgres>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    index(pool.clone())
        .or(board::get(pool.clone()))
        .or(post::get(pool.clone()))
        .or(post::create(pool.clone()))
        .or(post::reply(pool))
        .or(user::get())
        .or(assets())
}

pub fn index(
    pool: Pool<Postgres>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path::end()
        .and(warp::get())
        .and(db::with_pool(pool))
        .and_then(handlers::index)
}

pub fn assets() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("assets").and(warp::fs::dir("assets"))
}
