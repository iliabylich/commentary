use anyhow::{Context, Result};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Json, Response},
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;

use crate::{app_error::AppError, config::Config, resource::Asset, state::AppState};

pub(crate) struct Web;

impl Web {
    pub(crate) async fn spawn(state: AppState) -> Result<()> {
        let app = Router::new()
            .nest(
                "/commentary",
                Router::new()
                    .route("/index", get(Self::index_html))
                    .route("/index.mjs", get(Self::index_mjs))
                    .route("/output.css", get(Self::output_css))
                    .route("/comments.json", get(Self::comments_json))
                    .route("/leave-comment", post(Self::leave_comment)),
            )
            .with_state(state);

        let port = Config::global()?.listen_on;
        let listener = TcpListener::bind(format!("0.0.0.0:{port}"))
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

    async fn index_html() -> Result<Response, AppError> {
        Ok(Html(Asset::index_html()?).into_response())
    }

    async fn index_mjs() -> Result<Response, AppError> {
        Ok((
            StatusCode::OK,
            [("content-type", "text/javascript")],
            Asset::index_mjs()?,
        )
            .into_response())
    }

    async fn output_css() -> Result<Response, AppError> {
        Ok((
            StatusCode::OK,
            [("content-type", "text/css")],
            Asset::output_css()?,
        )
            .into_response())
    }

    async fn comments_json(
        State(state): State<AppState>,
        query: Query<PostId>,
    ) -> Result<Response, AppError> {
        let comments = state.database.get_comments(&query.post_id).await?;
        Ok(Json(comments).into_response())
    }

    async fn leave_comment(
        State(state): State<AppState>,
        Json(payload): Json<CreateComment>,
    ) -> Result<Response, AppError> {
        let comment = state
            .database
            .create_comment(&payload.author, &payload.body, &payload.post_id)
            .await?;
        Ok(Json(comment).into_response())
    }
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CreateComment {
    pub(crate) author: String,
    pub(crate) body: String,
    pub(crate) post_id: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PostId {
    pub(crate) post_id: String,
}
