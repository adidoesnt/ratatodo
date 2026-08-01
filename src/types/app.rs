use ratatui::{Frame, style::Style, widgets::{List, ListItem, ListState}};

use crate::types::todo::Todo;

pub struct App {
    pub running: bool,
    pub todos: Vec<Todo>,
    pub list_state: ListState,
}

impl App {
    fn format_todo(todo: &Todo) -> String {
        let checkbox = match todo.completed {
            true => "[x]",
            false => "[ ]"
        };
        let id = &todo.id;
        let title = &todo.title;
        let default_desc = String::from("No description provided.");
        let desc = &todo.description.as_ref().unwrap_or(&default_desc);

        format!("{checkbox} | {id}) {title}: {desc}")
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let list_items: Vec<ListItem> = self.todos.iter().map(|todo| {
            ListItem::new(App::format_todo(todo))
        }).collect();
        let list = List::new(list_items).highlight_style(Style::new().reversed());
        frame.render_stateful_widget(list, frame.area(), &mut self.list_state);
    }
}
