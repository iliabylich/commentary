use chrono::{offset::Utc, DateTime};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub(crate) struct State {
    state: Arc<RwLock<InnerState>>,
}

impl State {
    pub(crate) fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(InnerState::new())),
        }
    }

    pub(crate) async fn push(&self, post_slug: String, comment: Comment) {
        let mut guard = self.state.write().await;
        let comments = guard.data.entry(post_slug).or_insert_with(|| vec![]);
        comments.push(comment);
    }

    pub(crate) async fn get(&self, post_slug: &str) -> Vec<Comment> {
        let guard = self.state.read().await;
        let d = guard
            .data
            .get(post_slug)
            .map(|comments| comments.to_vec())
            .unwrap_or_default();
        d
    }
}

#[derive(Debug, Serialize)]
struct InnerState {
    data: HashMap<String, Vec<Comment>>,
}
impl InnerState {
    fn new() -> Self {
        Self {
            data: HashMap::from([(
                "test-slug".to_string(),
                vec![Comment::new("me".to_string(), "comment body".to_string())],
            )]),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
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
