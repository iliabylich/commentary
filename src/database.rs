use anyhow::{Context, Result};
use sqlx::sqlite::SqlitePool;

use crate::{comment::Comment, config::Config};

#[derive(Clone, Debug)]
pub(crate) struct Database {
    pub(crate) pool: SqlitePool,
}

fn database_url() -> Result<String> {
    let config = Config::global()?;
    Ok(format!("sqlite:{}", config.database_path))
}

impl Database {
    pub(crate) async fn new() -> Result<Self> {
        let pool = SqlitePool::connect(&database_url()?)
            .await
            .context("Failed to connect to sqlite")?;
        println!("Connected to sqlite");

        Ok(Self { pool })
    }

    pub(crate) async fn create_comment(
        &self,
        author: &str,
        body: &str,
        post_id: &str,
    ) -> Result<Comment> {
        let commant_id = sqlx::query(
            r#"
            INSERT INTO comments (author, body, post_id)
            VALUES (?, ?, ?)
        "#,
        )
        .bind(author)
        .bind(body)
        .bind(post_id)
        .execute(&self.pool)
        .await
        .context("Failed to insert a comment")?
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
        .context("Failed to fetch comment")
    }

    pub(crate) async fn get_comments(&self, post_id: &str) -> Result<Vec<Comment>> {
        sqlx::query_as::<_, Comment>(
            "SELECT * FROM comments WHERE post_id = ? ORDER BY created_at DESC",
        )
        .bind(post_id)
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch comments")
    }

    pub(crate) async fn get_new_comments(&self) -> Result<Vec<Comment>> {
        let comments = sqlx::query_as::<_, Comment>(
            "SELECT * FROM comments WHERE sent = FALSE ORDER BY created_at DESC",
        )
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch new comments")?;

        sqlx::query("UPDATE comments SET sent = TRUE WHERE sent = FALSE")
            .execute(&self.pool)
            .await
            .context("Failed to update comments")?;

        Ok(comments)
    }
}
