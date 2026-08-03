use crate::types::todo::Todo;

pub fn mock_todos() -> Vec<Todo> {
    vec![
        Todo {
            id: None,
            title: String::from("Learn ratatui basics"),
            completed: true,
        },
        Todo {
            id: None,
            title: String::from("Render todos as a List widget"),
            completed: false,
        },
        Todo {
            id: None,
            title: String::from("Wire up add/toggle/delete"),
            completed: false,
        },
    ]
}
