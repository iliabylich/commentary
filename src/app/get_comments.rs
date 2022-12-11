use crate::state::State;
use hyper::{header::CONTENT_TYPE, Body, Request, Response};

pub(crate) async fn get_comments(
    req: Request<Body>,
    state: State,
) -> Result<Response<Body>, Box<dyn std::error::Error>> {
    let slug = parse_slug(&req);
    let comments = state.get(slug).await;
    let body = serde_json::to_string(&comments)?;

    let res = Response::builder()
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(body))?;

    Ok(res)
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
