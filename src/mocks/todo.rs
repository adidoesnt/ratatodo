use crate::types::todo::Todo;

pub fn mock_todos() -> Vec<Todo> {
    vec![
        Todo {
            id: 0,
            title: String::from("Learn ratatui basics"),
            completed: true,
        },
        Todo {
            id: 1,
            title: String::from("Render todos as a List widget"),
            completed: false,
        },
        Todo {
            id: 2,
            title: String::from("Wire up add/toggle/delete"),
            completed: false,
        },
    ]
}
