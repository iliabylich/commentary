use crate::state::{Comment, State};
use hyper::{header::CONTENT_TYPE, Body, Request, Response, StatusCode};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct FormData {
    post_slug: String,
    author: String,
    body: String,
}

pub(crate) async fn post_comment(
    req: Request<Body>,
    state: State,
) -> Result<Response<Body>, Box<dyn std::error::Error>> {
    let body = hyper::body::to_bytes(req.into_body()).await?;

    let FormData {
        post_slug,
        author,
        body,
    } = serde_json::from_slice::<FormData>(&body)?;

    let comment = Comment::new(author, body);
    let json = serde_json::to_string(&comment)?;
    state.push(post_slug, comment).await;
    state.sync().await?;

    let res = Response::builder()
        .header(CONTENT_TYPE, "application/json")
        .status(StatusCode::CREATED)
        .body(Body::from(json))?;

    Ok(res)
}
