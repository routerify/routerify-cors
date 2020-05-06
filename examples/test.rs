use hyper::{Body, Request, Response, Server};
use routerify::{Router, RouterService};
use routerify_cors::enable_cors_all;
use std::{convert::Infallible, net::SocketAddr};

async fn home_handler(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Home page")))
}

fn router() -> Router<Body, Infallible> {
    Router::builder()
        .middleware(enable_cors_all())
        .get("/", home_handler)
        .build()
        .unwrap()
}

#[tokio::main]
async fn main() {
    let router = router();

    let service = RouterService::new(router).unwrap();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    let server = Server::bind(&addr).serve(service);

    println!("App is running on: {}", addr);
    if let Err(err) = server.await {
        eprintln!("Server error: {}", err);
    }
}
