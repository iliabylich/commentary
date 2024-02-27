use anyhow::{Context, Result};
use askama::Template;
use axum::{
    extract::{Query, State},
    response::{Html, IntoResponse, Json, Response},
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;

use crate::{
    app_error::AppError,
    config::Config,
    state::AppState,
    template::{CommentPartial, Index},
};

pub(crate) struct Web;

impl Web {
    pub(crate) async fn spawn(state: AppState) -> Result<()> {
        let app = Router::new()
            .nest(
                "/commentary",
                Router::new()
                    .route("/index", get(index_html))
                    .route("/leave-comment", post(leave_comment)),
            )
            .with_state(state);

        let port = Config::global()?.listen_on;
        let listener = TcpListener::bind(("0.0.0.0", port))
            .await
            .context("Failed to bind to port")?;
        println!(
            "Listening on {}",
            listener
                .local_addr()
                .context("Failed to get local address")?
        );

        axum::serve(listener, app)
            .await
            .context("Failed to spawn web server")?;

        Ok(())
    }
}

#[derive(serde::Deserialize)]
struct IndexParams {
    post_id: String,
}

async fn index_html(
    Query(query): Query<IndexParams>,
    State(state): State<AppState>,
) -> Result<Response, AppError> {
    let comments = state.database.get_comments(&query.post_id).await?;

    let html = Index {
        comments: &comments,
        post_id: &query.post_id,
    }
    .render()?;
    Ok(Html(html).into_response())
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LeaveCommentParams {
    pub(crate) author: String,
    pub(crate) body: String,
    pub(crate) post_id: String,
}

async fn leave_comment(
    State(state): State<AppState>,
    Json(params): Json<LeaveCommentParams>,
) -> Result<Response, AppError> {
    let comment = state
        .database
        .create_comment(&params.author, &params.body, &params.post_id)
        .await?;
    let html = CommentPartial { comment: &comment }.render()?;
    Ok(Html(html).into_response())
}
