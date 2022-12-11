use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use std::convert::Infallible;

mod app;
use app::app;

mod state;
use state::State;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:3000".parse()?;
    let state = State::new();

    let server = Server::bind(&addr).serve(make_service_fn(move |_conn| {
        let state = state.clone();
        async {
            Ok::<_, Infallible>(service_fn(move |req| {
                let state = state.clone();
                app(req, state)
            }))
        }
    }));

    println!("Starting server on http://127.0.0.1:3000");

    if let Err(e) = server.await {
        eprintln!("Failed to start server: {}", e);
    }

    Ok(())
}
