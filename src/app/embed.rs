use crate::state::State;
use hyper::{header::CONTENT_TYPE, Body, Request, Response};

pub(crate) fn embed(
    _req: Request<Body>,
    _state: State,
) -> Result<Response<Body>, Box<dyn std::error::Error>> {
    let body = if cfg!(debug_assertions) {
        Body::from(std::fs::read("embed.html")?)
    } else {
        Body::from(include_bytes!("../../embed.html") as &'static [u8])
    };

    let res = Response::builder()
        .header(CONTENT_TYPE, "text/html")
        .body(body)?;

    Ok(res)
}
