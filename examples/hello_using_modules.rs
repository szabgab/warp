#![deny(warnings)]

#[tokio::main]
async fn main() {
    let routes = filters::setup_routes();

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

mod filters {
    use super::handlers;
    use warp::Filter;

    pub fn setup_routes() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
    {
        warp::path!().and(warp::get()).and_then(handlers::say_hello)
    }
}

mod handlers {
    use std::convert::Infallible;

    pub async fn say_hello() -> Result<impl warp::Reply, Infallible> {
        Ok(warp::reply::html("Hello, <b>World</b>!"))
    }
}
