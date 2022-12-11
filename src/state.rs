use chrono::{offset::Utc, DateTime};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub(crate) struct State {
    state: Arc<Mutex<InnerState>>,
}

impl State {
    pub(crate) fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(InnerState::new())),
        }
    }

    pub(crate) fn push(&self, post_slug: String, comment: Comment) {
        let mut state = self.state.lock().unwrap();
        let comments = state.data.entry(post_slug).or_insert_with(|| vec![]);
        comments.push(comment)
    }

    pub(crate) fn get(&self, post_slug: &str) -> Vec<Comment> {
        let state = self.state.lock().unwrap();
        state
            .data
            .get(post_slug)
            .map(|comments| comments.to_vec())
            .unwrap_or_default()
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
