use axum::{
    extract::State,
    response::{Html, Json},
    routing::{get, post},
    Router, Server,
};
use std::net::SocketAddr;

use crate::{comment::Comment, config::Config, state::AppState};

pub(crate) struct Web;

impl Web {
    pub(crate) async fn spawn(state: AppState) {
        let app = Router::new()
            .nest(
                "/commentary",
                Router::new()
                    .route("/index", get(Self::index))
                    .route("/comments", get(Self::get_comments))
                    .route("/comment", post(Self::comment)),
            )
            .with_state(state);

        let addr = SocketAddr::from(([0, 0, 0, 0], Config::global().listen_on));
        println!("Listening on {}", addr);

        Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .expect("Failed to spawn web server");
    }

    async fn index() -> Html<&'static str> {
        static INDEX: &str = include_str!("../views/index.html");
        Html(INDEX)
    }

    async fn get_comments(State(state): State<AppState>) -> Json<Vec<Comment>> {
        let comments = state.database.get_comments().await;
        Json(comments)
    }

    async fn comment(
        State(state): State<AppState>,
        Json(payload): Json<CreateComment>,
    ) -> Json<Comment> {
        let comment = state
            .database
            .create_comment(&payload.author, &payload.body)
            .await;
        Json(comment)
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct CreateComment {
    pub(crate) author: String,
    pub(crate) body: String,
}
