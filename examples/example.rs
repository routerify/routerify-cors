use hyper::{Body, Request, Response, Server};
use routerify::{Router, RouterService};
// Import the CORS crate.
use routerify_cors::enable_cors_all;
use std::{convert::Infallible, net::SocketAddr};

// A handler for "/" page.
async fn home_handler(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Home page")))
}

// Create a router.
fn router() -> Router<Body, Infallible> {
    Router::builder()
        // Attach the CORS middleware.
        .middleware(enable_cors_all())
        .get("/", home_handler)
        .build()
        .unwrap()
}

#[tokio::main]
async fn main() {
    let router = router();

    // Create a Service from the router above to handle incoming requests.
    let service = RouterService::new(router).unwrap();

    // The address on which the server will be listening.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    // Create a server by passing the created service to `.serve` method.
    let server = Server::bind(&addr).serve(service);

    println!("App is running on: {}", addr);
    if let Err(err) = server.await {
        eprintln!("Server error: {}", err);
    }
}
