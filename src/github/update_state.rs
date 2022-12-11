use std::collections::HashMap;

use crate::github::{gist_id, github_token, GIST_NAME};
use hyper::{Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde::Serialize;

#[derive(Serialize)]
struct FormData {
    files: HashMap<&'static str, FileData>,
}

#[derive(Serialize)]
struct FileData {
    content: String,
}

pub(crate) async fn update_state(content: String) -> Result<(), Box<dyn std::error::Error>> {
    let data = FormData {
        files: HashMap::from([(GIST_NAME, FileData { content })]),
    };
    let data = serde_json::to_string(&data)?;

    let req = Request::patch(format!("https://api.github.com/gists/{}", gist_id()))
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {}", github_token()))
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "Ilya-Bylich-Comments-App")
        .body(Body::from(data))?;

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    client.request(req).await?;
    Ok(())
}
