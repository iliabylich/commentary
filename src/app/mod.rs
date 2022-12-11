use crate::state::State;
use hyper::{Body, Method, Request, Response, StatusCode};
use std::convert::Infallible;

mod alive;
use alive::alive;

mod embed;
use embed::embed;

mod get_comments;
use get_comments::get_comments;

mod post_comment;
use post_comment::post_comment;

pub(crate) async fn app(req: Request<Body>, state: State) -> Result<Response<Body>, Infallible> {
    eprintln!(
        "Request: {:?} {:?}",
        req.method(),
        req.uri().path_and_query(),
    );

    let res = router(req, state).await;

    eprintln!("Response: {:?}", res.status());

    Ok(res)
}

async fn router(req: Request<Body>, state: State) -> Response<Body> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => alive(req, state),

        (&Method::POST, "/comment") => post_comment(req, state).await,

        (&Method::GET, "/comments") => get_comments(req, state),

        (&Method::GET, "/embed") => embed(req, state),

        _ => {
            let mut res = Response::new(Body::empty());
            *res.status_mut() = StatusCode::NOT_FOUND;
            res
        }
    }
}
