use chrono::{offset::Utc, DateTime};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub(crate) struct State {
    state: Arc<RwLock<InnerState>>,
}

impl State {
    pub(crate) async fn push(&self, post_slug: String, comment: Comment) {
        let mut guard = self.state.write().await;
        let comments = guard.data.entry(post_slug).or_insert_with(|| vec![]);
        comments.push(comment);
    }

    pub(crate) async fn get(&self, post_slug: &str) -> Vec<Comment> {
        let guard = self.state.read().await;
        guard
            .data
            .get(post_slug)
            .map(|comments| comments.to_vec())
            .unwrap_or_default()
    }

    pub(crate) async fn initial() -> Self {
        let json = crate::github::read_initial_state().await;
        let state = serde_json::from_str(&json).unwrap();
        Self {
            state: Arc::new(RwLock::new(state)),
        }
    }

    pub(crate) async fn debug(&self) -> String {
        let guard = self.state.read().await;
        format!("{:?}", *guard)
    }

    pub(crate) async fn sync(&self) {
        let guard = self.state.read().await;
        let content = serde_json::to_string_pretty(&*guard).unwrap();
        crate::github::update_state(content).await
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct InnerState {
    data: HashMap<String, Vec<Comment>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Comment {
    pub(crate) author: String,
    pub(crate) body: String,
    pub(crate) date: DateTime<Utc>,
}

impl Comment {
    pub(crate) fn new(author: String, body: String) -> Self {
        Self {
            author,
            body,
            date: Utc::now(),
        }
    }
}
