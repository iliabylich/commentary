use crate::state::State;
use hyper::{header::CONTENT_TYPE, Body, Request, Response};

pub(crate) async fn get_comments(req: Request<Body>, state: State) -> Response<Body> {
    let slug = parse_slug(&req);
    let comments = state.get(slug).await;
    let body = serde_json::to_string(&comments).unwrap();

    Response::builder()
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(body))
        .unwrap()
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
