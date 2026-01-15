use sqlx::{Pool, Postgres};
use warp::Filter;

use crate::{db, handlers};

pub fn get(
    pool: Pool<Postgres>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("b" / String / i64)
        .and(warp::get())
        .and(db::with_pool(pool))
        .and_then(handlers::post::get)
}

pub fn create(
    pool: Pool<Postgres>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("b" / String)
        .and(warp::post())
        .and(warp::body::form())
        .and(warp::body::content_length_limit(1024 * 4))
        .and(db::with_pool(pool))
        .and_then(
            |name, data, pool| async move { handlers::post::create(name, None, data, pool).await },
        )
}

pub fn reply(
    pool: Pool<Postgres>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("b" / String / i64)
        .and(warp::post())
        .and(warp::body::form())
        .and(warp::body::content_length_limit(1024 * 4))
        .and(db::with_pool(pool))
        .and_then(|name, parent, data, pool| async move {
            handlers::post::create(name, Some(parent), data, pool).await
        })
}
