#![deny(warnings)]
use warp::Filter;

#[tokio::main]
async fn main() {
    // Match any request and return hello world!
    let routes = warp::any()
        .map(|| warp::reply::with_header("<b>Hello</b>, World!", "Content-Type", "text/html"));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
