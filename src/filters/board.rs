use sqlx::{Pool, Postgres};
use warp::Filter;

use crate::{db, handlers};

pub fn all(
    pool: Pool<Postgres>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path::end()
        .and(warp::get())
        .and(db::with_pool(pool))
        .and_then(handlers::index)
}

pub fn get(
    pool: Pool<Postgres>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("b" / String)
        .and(warp::get())
        .and(warp::path::end())
        .and(db::with_pool(pool))
        .and_then(handlers::board::get)
}
