use crate::state::State;
use hyper::{Body, Request, Response};

pub(crate) fn alive(
    _req: Request<Body>,
    _state: State,
) -> Result<Response<Body>, Box<dyn std::error::Error>> {
    Ok(Response::new(Body::from("Alive")))
}
