use crate::state::State;
use hyper::{
    header::{HeaderValue, CONTENT_TYPE},
    Body, Request, Response,
};
use std::sync::{Arc, Mutex};

pub(crate) async fn post_comment(req: Request<Body>, state: Arc<Mutex<State>>) -> Response<Body> {
    let mut res = Response::new(Body::empty());
    res.headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let body = hyper::body::to_bytes(req.into_body()).await;

    todo!()
}
