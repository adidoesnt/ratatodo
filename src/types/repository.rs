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

    pub async fn add_todo(&self, title: &str) -> Result<Todo> {
        sqlx::query_as::<_, Todo>(
            "INSERT INTO todos (title, completed) VALUES (?, false) RETURNING id, title, completed",
        )
        .bind(title)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| std::io::Error::other(format!("Failed to add todo: {e}")))
    }

    pub async fn toggle_todo(&self, id: i64, completed: bool) -> Result<()> {
        sqlx::query("UPDATE todos SET completed = ? WHERE id = ?")
            .bind(completed)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| std::io::Error::other(format!("Failed to toggle todo: {e}")))?;
        Ok(())
    }

    pub async fn delete_todo(&self, id: i64) -> Result<()> {
        sqlx::query("DELETE FROM todos WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| std::io::Error::other(format!("Failed to delete todo: {e}")))?;
        Ok(())
    }
}
