use crate::state::State;
use hyper::{Body, Request, Response};

pub(crate) fn alive(_req: Request<Body>, _state: State) -> Response<Body> {
    Response::new(Body::from("Alive"))
}
