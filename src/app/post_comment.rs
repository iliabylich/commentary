use crate::state::{Comment, State};
use hyper::{
    header::{HeaderValue, CONTENT_TYPE},
    Body, Request, Response, StatusCode,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct FormData {
    post_slug: String,
    author: String,
    body: String,
}

pub(crate) async fn post_comment(req: Request<Body>, state: State) -> Response<Body> {
    let body = match hyper::body::to_bytes(req.into_body()).await {
        Ok(bytes) => bytes,
        Err(e) => return internal_server_error(e),
    };

    let FormData {
        post_slug,
        author,
        body,
    } = match serde_json::from_slice::<FormData>(&body) {
        Ok(data) => data,
        Err(e) => return internal_server_error(e),
    };

    let comment = Comment::new(author, body);
    let json = serde_json::to_string(&comment).unwrap();
    state.push(post_slug, comment).await;
    state.sync().await;

    let mut res = Response::new(Body::empty());
    res.headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    *res.status_mut() = StatusCode::CREATED;
    *res.body_mut() = Body::from(json);
    res
}

fn internal_server_error<E: std::fmt::Debug>(e: E) -> Response<Body> {
    eprintln!("Internal Error: {:?}", e);
    let mut res = Response::new(Body::empty());
    res.headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    *res.body_mut() = Body::from("{}");
    *res.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
    res
}
