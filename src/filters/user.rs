use warp::Filter;

use crate::views;

pub fn get() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("u")
        .and(warp::path::param())
        .map(|param: String| views::page(&format!("user {}", param)))
}
