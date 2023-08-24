use hyper::{Body, Response, StatusCode};
use std::convert::Infallible;

pub fn handle_request() -> Result<Response<Body>, Infallible> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("Not Found"))
        .unwrap())
}
