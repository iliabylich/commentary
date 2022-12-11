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

    let uri = format!("http://example.com{}", req.uri().to_string());
    let url = Url::parse(&uri).unwrap();
    let mut slug = None;
    for (key, value) in url.query_pairs() {
        if key == "slug" {
            slug = Some(value);
        }
    }
    if let Some(slug) = slug {
        if let Some(comments) = state.lock().unwrap().posts.get(slug.as_ref()) {
            *res.body_mut() = Body::from(serde_json::to_string(comments).unwrap());
            return res;
        }
    }

    *res.body_mut() = Body::from("[]");
    res
}
