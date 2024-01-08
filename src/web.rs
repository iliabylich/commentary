use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Json},
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;

use crate::{comment::Comment, config::Config, resource::Asset, state::AppState};

pub(crate) struct Web;

impl Web {
    pub(crate) async fn spawn(state: AppState) {
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

        let port = Config::global().listen_on;
        let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
        println!("Listening on {}", listener.local_addr().unwrap());

        axum::serve(listener, app)
            .await
            .expect("Failed to spawn web server");
    }

    async fn index_html() -> Html<String> {
        Html(Asset::index_html())
    }

    async fn index_mjs() -> impl IntoResponse {
        (
            StatusCode::OK,
            [("content-type", "text/javascript")],
            Asset::index_mjs(),
        )
    }

    async fn output_css() -> impl IntoResponse {
        (
            StatusCode::OK,
            [("content-type", "text/css")],
            Asset::output_css(),
        )
    }

    async fn comments_json(
        State(state): State<AppState>,
        query: Query<PostId>,
    ) -> Json<Vec<Comment>> {
        let comments = state.database.get_comments(&query.post_id).await;
        Json(comments)
    }

    async fn leave_comment(
        State(state): State<AppState>,
        Json(payload): Json<CreateComment>,
    ) -> Json<Comment> {
        let comment = state
            .database
            .create_comment(&payload.author, &payload.body, &payload.post_id)
            .await;
        Json(comment)
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
