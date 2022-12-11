use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use std::convert::Infallible;
use std::net::SocketAddr;

mod app;
use app::app;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let server = Server::bind(&addr).serve(make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(app))
    }));

    println!("Starting server on http://127.0.0.1:3000");

    if let Err(e) = server.await {
        eprintln!("Failed to start server: {}", e);
    }
}
