#[derive(Debug, sqlx::FromRow)]
pub struct Todo {
    pub id: Option<i64>,
    pub title: String,
    pub completed: bool,
}
