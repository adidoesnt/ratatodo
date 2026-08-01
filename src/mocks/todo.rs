use crate::types::todo::Todo;

pub fn mock_todos() -> Vec<Todo> {
    vec![
        Todo {
            title: String::from("Learn ratatui basics"),
            completed: true,
        },
        Todo {
            title: String::from("Render todos as a List widget"),
            completed: false,
        },
        Todo {
            title: String::from("Wire up add/toggle/delete"),
            completed: false,
        },
    ]
}
