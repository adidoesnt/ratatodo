use crate::types::todo::Todo;

pub fn mock_todos() -> Vec<Todo> {
    vec![
        Todo {
            id: 0,
            title: String::from("Learn ratatui basics"),
            description: Some(String::from(
                "Read through the hello-world template and understand the draw loop",
            )),
            completed: true,
        },
        Todo {
            id: 1,
            title: String::from("Render todos as a List widget"),
            description: None,
            completed: false,
        },
        Todo {
            id: 2,
            title: String::from("Wire up add/toggle/delete"),
            description: Some(String::from("Handle key events to mutate the todo list")),
            completed: false,
        },
    ]
}
