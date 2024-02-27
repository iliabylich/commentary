use askama::Template;

use crate::comment::Comment;

#[derive(Template)]
#[template(path = "index.html")]
pub(crate) struct Index<'a> {
    pub(crate) comments: &'a [Comment],
    pub(crate) post_id: &'a str,
}

#[derive(Template)]
#[template(path = "_comment.html")]
pub(crate) struct CommentPartial<'a> {
    pub(crate) comment: &'a Comment,
}
