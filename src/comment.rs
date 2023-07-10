use crate::database::Database;

#[derive(sqlx::FromRow, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct Comment {
    pub(crate) id: i64,
    pub(crate) author: String,
    pub(crate) body: String,
    pub(crate) created_at: chrono::DateTime<chrono::Utc>,
    pub(crate) post_id: String,
    pub(crate) sent: bool,
}

const CRETE_COMMENTS_TABLE_SQL: &str = r#"
    CREATE TABLE IF NOT EXISTS comments (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        author TEXT NOT NULL,
        body TEXT NOT NULL,
        created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
        post_id TEXT NOT NULL,
        sent BOOLEAN NOT NULL DEFAULT FALSE
    )
"#;

impl Comment {
    pub(crate) async fn create_table(database: &Database) {
        sqlx::query(CRETE_COMMENTS_TABLE_SQL)
            .execute(&database.pool)
            .await
            .expect("failed to create `comments` table");
        println!("Created `comments` table");
    }
}
