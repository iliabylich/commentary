use crate::state::State;
use hyper::{
    header::{HeaderValue, CONTENT_TYPE},
    Body, Request, Response,
};
use std::sync::{Arc, Mutex};
use url::Url;

pub(crate) fn get_comments(req: &Request<Body>, state: Arc<Mutex<State>>) -> Response<Body> {
    let mut res = Response::new(Body::empty());
    res.headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    if let Some(slug) = parse_slug(req) {
        if let Some(comments) = state.lock().unwrap().posts.get(&slug) {
            *res.body_mut() = Body::from(serde_json::to_string(comments).unwrap());
            return res;
        }
    }

    *res.body_mut() = Body::from("[]");
    res
}

fn parse_slug(req: &Request<Body>) -> Option<String> {
    let uri = format!("http://example.com{}", req.uri().to_string());
    let url = Url::parse(&uri).ok()?;
    for (key, value) in url.query_pairs() {
        if key == "slug" {
            return Some(value.into_owned());
        }
    }
    None
}
