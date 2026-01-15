use crate::db::{self, with_pool};
use crate::handlers;
use crate::views::page;
use sqlx::{Pool, Postgres};
use warp::Filter;

// All the routes combined
pub fn api(
    pool: Pool<Postgres>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    boards_get(pool.clone())
        .or(user_get())
        .or(board_get(pool.clone()))
        .or(thread_get(pool.clone()))
        .or(thread_post(pool.clone()))
        .or(reply_post(pool))
        .or(static_assets())
}

pub fn boards_get(
    pool: Pool<Postgres>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path::end()
        .and(warp::get())
        .and(with_pool(pool))
        .and_then(handlers::index)
}

pub fn user_get() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("u")
        .and(warp::path::param())
        .map(|param: String| page(&format!("user {}", param)))
}

pub fn board_get(
    pool: Pool<Postgres>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("b" / String)
        .and(warp::get())
        .and(warp::path::end())
        .and(with_pool(pool))
        .and_then(handlers::board::get)
}

pub fn thread_get(
    pool: Pool<Postgres>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("b" / String / i64)
        .and(warp::get())
        .and(with_pool(pool))
        .and_then(handlers::post::get)
}

pub fn thread_post(
    pool: Pool<Postgres>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("b" / String)
        .and(warp::post())
        .and(warp::body::form())
        .and(warp::body::content_length_limit(1024 * 4))
        .and(with_pool(pool))
        .and_then(
            |name, data, pool| async move { handlers::post::create(name, None, data, pool).await },
        )
}

// TODO: stop repetition
pub fn reply_post(
    pool: Pool<Postgres>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("b" / String / i64)
        .and(warp::post())
        .and(warp::body::form())
        .and(warp::body::content_length_limit(1024 * 4))
        .and(with_pool(pool))
        .and_then(|name, parent, data, pool| async move {
            handlers::post::create(name, Some(parent), data, pool).await
        })
}

pub fn static_assets() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    warp::path("assets").and(warp::fs::dir("assets"))
}
