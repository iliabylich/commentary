use sqlx::sqlite::SqlitePool;

use crate::{comment::Comment, config::Config};

#[derive(Clone, Debug)]
pub(crate) struct Database {
    pub(crate) pool: SqlitePool,
}

fn database_url() -> String {
    let config = Config::global();
    format!("sqlite:{}", config.database_path)
}

impl Database {
    pub(crate) async fn new() -> Self {
        let pool = SqlitePool::connect(&database_url())
            .await
            .expect("Failed to connect to sqlite");
        println!("Connected to sqlite");

        Self { pool }
    }

    pub(crate) async fn create_comment(&self, author: &str, body: &str) -> Comment {
        let commant_id = sqlx::query(
            r#"
            INSERT INTO comments (author, body)
            VALUES (?, ?)
        "#,
        )
        .bind(author)
        .bind(body)
        .execute(&self.pool)
        .await
        .expect("Failed to insert a comment")
        .last_insert_rowid();

        sqlx::query_as::<_, Comment>(
            r#"
            SELECT * FROM comments
            WHERE id = ?
            "#,
        )
        .bind(commant_id)
        .fetch_one(&self.pool)
        .await
        .expect("Failed to fetch comment")
    }

    pub(crate) async fn get_comments(&self) -> Vec<Comment> {
        sqlx::query_as::<_, Comment>("SELECT * FROM comments")
            .fetch_all(&self.pool)
            .await
            .expect("Failed to fetch comments")
    }
}
