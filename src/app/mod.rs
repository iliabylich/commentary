use crate::state::State;
use hyper::{Body, Method, Request, Response, StatusCode};
use std::convert::Infallible;
use std::sync::{Arc, Mutex};

mod alive;
use alive::alive;

mod frame;
// use frame::frame;

mod get_comments;
use get_comments::get_comments;

mod post_comment;
// use post_comment::post_comment;

pub(crate) async fn app(
    req: Request<Body>,
    state: Arc<Mutex<State>>,
) -> Result<Response<Body>, Infallible> {
    let mut res;

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => res = alive(&req, state),

        (&Method::POST, "/comment") => {
            todo!()
        }

        (&Method::GET, "/comments") => res = get_comments(&req, state),

        (&Method::GET, "/frame") => {
            todo!()
        }

        _ => {
            res = Response::new(Body::empty());
            *res.status_mut() = StatusCode::NOT_FOUND;
        }
    }

    eprintln!(
        "Request: {:?} {:?}; Response: {:?}",
        req.method(),
        req.uri().path_and_query(),
        res.status()
    );

    Ok(res)
}
