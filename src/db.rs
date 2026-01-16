use sqlx::{Pool, Postgres};
use std::convert::Infallible;
use warp::Filter;

pub mod board;
pub mod post;

pub fn with_pool(
    pool: Pool<Postgres>,
) -> impl Filter<Extract = (Pool<Postgres>,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}
