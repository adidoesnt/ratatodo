use std::io::Result;

use sqlx::SqlitePool;

use crate::types::todo::Todo;

pub struct Repository {
    pub pool: SqlitePool,
}

impl Repository {
    pub async fn new() -> std::io::Result<Self> {
        let pool = SqlitePool::connect("sqlite::ratatodo.db")
            .await
            .map_err(|e| std::io::Error::other(format!("Failed to connect to database: {e}")))?;

        Ok(Self { pool })
    }

    pub async fn get_all_todos(&self) -> Result<Vec<Todo>> {
        sqlx::query_as::<_, Todo>("SELECT id, title, completed FROM todos")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| std::io::Error::other(format!("Failed to fetch todos: {e}")))
    }
}
