use crate::state::State;
use hyper::{Body, Request, Response};
use std::sync::{Arc, Mutex};

pub(crate) fn embed(_req: Request<Body>, _state: Arc<Mutex<State>>) -> Response<Body> {
    todo!()
}
