mod route;
mod util;

use hyper::{Server, Body, Request, Response, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&hyper::Method::GET, "/api") => route::api::handle_request(req).await,
        (&hyper::Method::GET, "/health") => route::health::handle_request(),
        _ => route::fallback::handle_request(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([0, 0, 0, 0], 8080).into();
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle_request))
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Server running on http://{}", addr);

    server.await?;

    Ok(())
}
