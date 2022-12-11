use crate::state::State;
use hyper::{
    header::{HeaderValue, CONTENT_TYPE},
    Body, Request, Response,
};
use url::Url;

pub(crate) async fn get_comments(req: Request<Body>, state: State) -> Response<Body> {
    let mut res = Response::new(Body::empty());
    res.headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let post_slug = parse_slug(&req);
    let comments = state.get(&post_slug).await;
    *res.body_mut() = Body::from(serde_json::to_string(&comments).unwrap());
    res
}

fn parse_slug(req: &Request<Body>) -> String {
    let uri = format!("http://example.com{}", req.uri().to_string());
    let url = Url::parse(&uri).unwrap();
    for (key, value) in url.query_pairs() {
        if key == "slug" {
            return value.into_owned();
        }
    }
    String::from("")
}
