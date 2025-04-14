#![deny(warnings)]

use std::convert::Infallible;

use warp::Filter;

#[tokio::main]
async fn main() {
    let routes = setup_routes();

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

pub fn setup_routes() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!().and(warp::get()).and_then(say_hello)
}

pub async fn say_hello() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::html("Hello, <b>World</b>!"))
}

