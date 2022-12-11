use crate::github::{gist_id, github_token, GIST_NAME};
use hyper::{Body, Client, Request};
use hyper_tls::HttpsConnector;

#[derive(Debug)]
struct MalformedGistContent;
impl std::fmt::Display for MalformedGistContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MalformedGistContent")
    }
}
impl std::error::Error for MalformedGistContent {}

pub(crate) async fn read_initial_state() -> Result<String, Box<dyn std::error::Error>> {
    let req = Request::get(format!("https://api.github.com/gists/{}", gist_id()))
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {}", github_token()))
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "Ilya-Bylich-Comments-App")
        .body(Body::from(""))?;

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let res = client.request(req).await?;

    let bytes = hyper::body::to_bytes(res.into_body()).await?;
    let data: serde_json::Value = serde_json::from_slice(&bytes)?;
    let content = data["files"][GIST_NAME]["content"]
        .as_str()
        .ok_or(MalformedGistContent)?;
    Ok(content.to_string())
}
