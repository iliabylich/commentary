use hyper::{Body, Method, Request, Response, StatusCode};
use std::convert::Infallible;

pub(crate) async fn app(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => *response.body_mut() = Body::from("Alive"),
        (&Method::POST, "/comment") => {
            todo!()
        }
        (&Method::GET, "/comments") => {
            todo!()
        }
        (&Method::GET, "/frame") => {
            todo!()
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    }

    eprintln!(
        "Request: {:?} {:?}; Response: {:?}",
        req.method(),
        req.uri().path_and_query(),
        response.status()
    );

    Ok(response)
}
