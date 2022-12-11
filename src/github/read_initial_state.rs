use crate::github::{gist_id, github_token, GIST_NAME};
use hyper::{Body, Client, Request};
use hyper_tls::HttpsConnector;

pub(crate) async fn read_initial_state() -> String {
    let req = Request::get(format!("https://api.github.com/gists/{}", gist_id()))
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {}", github_token()))
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "Ilya-Bylich-Comments-App")
        .body(Body::from(""))
        .unwrap();

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let res = client.request(req).await.unwrap();

    let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
    let data: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let content = data["files"][GIST_NAME]["content"].as_str().unwrap();
    content.to_string()
}
