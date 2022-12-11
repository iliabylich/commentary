pub(crate) fn gist_id() -> &'static str {
    include_str!("../../.gist-id").trim_end()
}

pub(crate) fn github_token() -> &'static str {
    include_str!("../../.github-token").trim_end()
}

pub(crate) const GIST_NAME: &str = "comments.json";

mod read_initial_state;
pub(crate) use read_initial_state::read_initial_state;

mod update_state;
pub(crate) use update_state::update_state;
