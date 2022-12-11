use crate::state::State;
use hyper::{
    header::{HeaderValue, CONTENT_TYPE},
    Body, Request, Response,
};

pub(crate) fn embed(_req: Request<Body>, _state: State) -> Response<Body> {
    let mut res = if cfg!(debug_assertions) {
        Response::new(Body::from(std::fs::read("embed.html").unwrap()))
    } else {
        Response::new(Body::from(
            include_bytes!("../../embed.html") as &'static [u8]
        ))
    };

    res.headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("text/html"));

    res
}
