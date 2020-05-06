[![Github Actions Status](https://github.com/routerify/routerify-cors/workflows/Test/badge.svg)](https://github.com/routerify/routerify-cors/actions)
[![crates.io](https://img.shields.io/crates/v/routerify-cors.svg)](https://crates.io/crates/routerify-cors)
[![Documentation](https://docs.rs/routerify-cors/badge.svg)](https://docs.rs/routerify-cors)
[![MIT](https://img.shields.io/crates/l/routerify-cors.svg)](./LICENSE)

# routerify-cors

A [`Routerify`](https://github.com/routerify/routerify) middleware which enables [`CORS`](https://en.wikipedia.org/wiki/Cross-origin_resource_sharing).

[Docs](https://docs.rs/routerify-cors)

## Install

Add this to your `Cargo.toml`:

```toml
[dependencies]
routerify = "1.1"
routerify-cors = "1.1"
```

## Example

```rust
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
    let service = RouterService::new(router);

    // The address on which the server will be listening.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    // Create a server by passing the created service to `.serve` method.
    let server = Server::bind(&addr).serve(service);

    println!("App is running on: {}", addr);
    if let Err(err) = server.await {
        eprintln!("Server error: {}", err);
    }
}

```

## Contributing

Your PRs and suggestions are always welcome.