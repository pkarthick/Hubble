#![deny(warnings)]

extern crate fs_utilities;
extern crate warp;
use warp::Filter;

mod filters {
    use crate::{fs_utilities::Request, handlers};
    use warp::Filter;

    pub fn get_folders() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        warp::path!("folder")
            .and(warp::post())
            .and(json_body())
            .and_then(handlers::list_folders)
    }

    pub fn get_size() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        warp::path!("size")
            .and(warp::post())
            .and(json_body())
            .and_then(handlers::get_size)
    }

    fn json_body() -> impl Filter<Extract = (Request,), Error = warp::Rejection> + Clone {
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}

mod handlers {

    use crate::fs_utilities::{get_folder_contents, get_folder_size, Folder, Request};
    use std::convert::Infallible;

    pub async fn list_folders(request: Request) -> Result<impl warp::Reply, Infallible> {
        // Just return a JSON array of todos, applying the limit and offset.
        println!("{:?}", request);
        let folder: Folder = get_folder_contents(request).await;
        Ok(warp::reply::json(&folder))
    }

    pub async fn get_size(request: Request) -> Result<impl warp::Reply, Infallible> {
        // Just return a JSON array of todos, applying the limit and offset.
        println!("{:?}", request);
        let size = get_folder_size(request).await;
        Ok(warp::reply::json(&size))
    }

}

#[tokio::main]
async fn main() {
    pretty_env_logger::init(); // Match any request and return hello world!
    let cors = warp::cors()
        .allow_any_origin()
        // .allow_origin("http://localhost:5555")
        .allow_methods(vec!["GET", "POST", "OPTIONS"])
        .allow_headers(vec![
            http::header::AUTHORIZATION,
            http::header::ACCEPT,
            http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
            http::header::ACCESS_CONTROL_ALLOW_HEADERS,
            http::header::ACCESS_CONTROL_REQUEST_HEADERS,
            http::header::ACCESS_CONTROL_REQUEST_METHOD,
        ])
        // .allow_header(http::header::CONTENT_TYPE)
        .max_age(3600);

    let dir = warp::path("public").and(warp::fs::dir("./public"));

    let sizeroute = filters::get_size();
    let routes = sizeroute.or(filters::get_folders()).with(cors).or(dir);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
