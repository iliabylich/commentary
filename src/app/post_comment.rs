use crate::state::{Comment, State};
use hyper::{header::CONTENT_TYPE, Body, Request, Response, StatusCode};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct FormData {
    post_slug: String,
    author: String,
    body: String,
}

pub(crate) async fn post_comment(req: Request<Body>, state: State) -> Response<Body> {
    let body = hyper::body::to_bytes(req.into_body()).await.unwrap();

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

    Response::builder()
        .header(CONTENT_TYPE, "application/json")
        .status(StatusCode::CREATED)
        .body(Body::from(json))
        .unwrap()
}

fn internal_server_error<E: std::fmt::Debug>(e: E) -> Response<Body> {
    eprintln!("Internal Error: {:?}", e);

    Response::builder()
        .header(CONTENT_TYPE, "application/json")
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from("{}"))
        .unwrap()
}
