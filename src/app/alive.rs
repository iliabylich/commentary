use crate::state::State;
use hyper::{Body, Request, Response};
use std::sync::{Arc, Mutex};

pub(crate) fn alive(_req: Request<Body>, _state: Arc<Mutex<State>>) -> Response<Body> {
    let mut res = Response::new(Body::empty());
    *res.body_mut() = Body::from("Alive");
    res
}
