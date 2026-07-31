#[derive(Debug)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
}
