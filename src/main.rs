// This file is the entry point of the application. It initializes the server, sets up middleware, and defines the main application logic.

use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;

mod config;
mod db;
mod handlers;
mod models;
mod routes;
mod services;
mod utils;

#[tokio::main]
async fn main() {
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle_request)) });

    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // Placeholder for request handling logic
    Ok(Response::new(Body::from("Hello, CMS!")))
}