use crate::state::State;
use hyper::{
    header::{HeaderValue, CONTENT_TYPE},
    Body, Request, Response,
};

pub(crate) async fn get_comments(req: Request<Body>, state: State) -> Response<Body> {
    let mut res = Response::new(Body::empty());
    res.headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let post_slug = parse_slug(&req);
    let comments = state.get(post_slug).await;
    *res.body_mut() = Body::from(serde_json::to_string(&comments).unwrap());
    res
}

fn parse_slug(req: &Request<Body>) -> &str {
    for param in req.uri().query().unwrap_or_default().split("&") {
        let mut parts = param.split("=");
        let key = parts.next();
        let value = parts.next();

        match (key, value) {
            (Some("slug"), Some(value)) => return value,
            _ => {}
        }
    }

    ""
}
