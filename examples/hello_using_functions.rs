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

mod test_hello_using_functions {
    #[tokio::test]
    async fn test_hello() {
        use super::setup_routes;
        use warp::test::request;
        use warp::http::StatusCode;

        let routes = setup_routes();
    
        let response = request()
            .method("GET")
            .path("/")
            .reply(&routes)
            .await;
    
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.body(), "Hello, <b>World</b>!");
    }

    #[tokio::test]
    async fn test_other_path() {
        use super::setup_routes;
        use warp::test::request;
        use warp::http::StatusCode;

        let routes = setup_routes();
    
        let response = request()
            .method("GET")
            .path("/hi")
            .reply(&routes)
            .await;
    
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        assert_eq!(response.body(), "");
    }
 
    #[tokio::test]
    async fn test_post_method() {
        use super::setup_routes;
        use warp::test::request;
        use warp::http::StatusCode;

        let routes = setup_routes();
    
        let response = request()
            .method("POST")
            .path("/")
            .reply(&routes)
            .await;
    
        assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
        assert_eq!(response.body(), "HTTP method not allowed");
    }

}
