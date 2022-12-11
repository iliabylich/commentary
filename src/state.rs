use chrono::{offset::Utc, DateTime};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub(crate) struct State {
    pub(crate) posts: HashMap<String, Vec<Comment>>,
}

impl State {
    pub(crate) fn new() -> Self {
        Self {
            posts: HashMap::from([(
                "test-slug".to_string(),
                vec![Comment::new("me".to_string(), "comment body".to_string())],
            )]),
        }
    }

    pub(crate) fn push(&mut self, post_slug: String, comment: Comment) {
        let comments = self.posts.entry(post_slug).or_insert_with(|| vec![]);
        comments.push(comment)
    }
}

#[derive(Debug, Serialize)]
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
