#[derive(Debug, sqlx::FromRow)]
pub struct Todo {
    pub title: String,
    pub completed: bool,
}
