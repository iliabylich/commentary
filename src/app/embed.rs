use crate::state::State;
use hyper::{Body, Request, Response};

pub(crate) fn embed(_req: Request<Body>, _state: State) -> Response<Body> {
    todo!()
}
