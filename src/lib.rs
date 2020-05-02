//! A [`Routerify`](https://github.com/routerify/routerify) middleware which enables [`CORS`](https://en.wikipedia.org/wiki/Cross-origin_resource_sharing).
//!
//! # Examples
//!
//! ```no_run
//! use hyper::{Body, Request, Response, Server};
//! use routerify::{Router, RouterService};
//! // Import the CORS crate.
//! use routerify_cors::enable_cors_all;
//! use std::{convert::Infallible, net::SocketAddr};
//!
//! // A handler for "/" page.
//! async fn home_handler(_: Request<Body>) -> Result<Response<Body>, Infallible> {
//!     Ok(Response::new(Body::from("Home page")))
//! }
//!
//! // Create a router.
//! fn router() -> Router<Body, Infallible> {
//!     Router::builder()
//!         // Attach the CORS middleware.
//!        .middleware(enable_cors_all())
//!        .get("/", home_handler)
//!         .build()
//!        .unwrap()
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     let router = router();
//!
//!     // Create a Service from the router above to handle incoming requests.
//!     let service = RouterService::new(router);
//!
//!     // The address on which the server will be listening.
//!     let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
//!
//!     // Create a server by passing the created service to `.serve` method.
//!     let server = Server::bind(&addr).serve(service);
//!
//!     println!("App is running on: {}", addr);
//!     if let Err(err) = server.await {
//!         eprintln!("Server error: {}", err);
//!     }
//! }
//! ```

use hyper::header::{self, HeaderValue};
use hyper::{body::HttpBody, Response};
use routerify::Middleware;

/// Enables [`CORS`](https://en.wikipedia.org/wiki/Cross-origin_resource_sharing) for all routes.
///
/// # Examples
///
/// ```
/// use hyper::{Body, Request, Response, Server};
/// use routerify::{Router, RouterService};
/// // Import the CORS crate.
/// use routerify_cors::enable_cors_all;
/// use std::{convert::Infallible, net::SocketAddr};
///
/// // A handler for "/" page.
/// async fn home_handler(_: Request<Body>) -> Result<Response<Body>, Infallible> {
///     Ok(Response::new(Body::from("Home page")))
/// }
///
/// # fn run() -> Router<Body, Infallible> {
/// // Create a router.
/// Router::builder()
///   // Attach the CORS middleware.
///   .middleware(enable_cors_all())
///   .get("/", home_handler)
///   .build()
///   .unwrap()
/// # }
///
/// # run();
/// ```
pub fn enable_cors_all<B, E>() -> Middleware<B, E>
where
    B: HttpBody + Send + Sync + Unpin + 'static,
    E: std::error::Error + Send + Sync + Unpin + 'static,
{
    Middleware::post(enable_cors_all_middleware_handler::<B, E>)
}

async fn enable_cors_all_middleware_handler<B, E>(mut res: Response<B>) -> Result<Response<B>, E>
where
    B: HttpBody + Send + Sync + Unpin + 'static,
    E: std::error::Error + Send + Sync + Unpin + 'static,
{
    let headers = res.headers_mut();

    headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
    headers.insert(header::ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("*"));
    headers.insert(header::ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("*"));

    Ok(res)
}
