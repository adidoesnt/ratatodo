use ratatui::{
    Frame, style::Style, widgets::{Block, List, ListItem, ListState},
};

use crate::types::todo::Todo;

pub enum Mode {
    Normal,
    Insert,
}

pub enum NavigationType {
    Increment,
    Decrement,
}

pub struct App {
    pub running: bool,
    pub mode: Mode,
    pub todos: Vec<Todo>,
    pub list_state: ListState,
}

impl App {
    fn format_todo(todo: &Todo) -> String {
        let checkbox = match todo.completed {
            true => "[x]",
            false => "[ ]",
        };
        let id = &todo.id;
        let title = &todo.title;
        let default_desc = String::from("No description provided.");
        let desc = &todo.description.as_ref().unwrap_or(&default_desc);

        format!("{checkbox} | {id}) {title}: {desc}")
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn navigate(&mut self, nav_type: NavigationType) {
        match nav_type {
            NavigationType::Increment => {
                self.list_state.select_next();
            }
            NavigationType::Decrement => {
                self.list_state.select_previous();
            }
        }
    }

    pub fn toggle_current_todo(&mut self) {
        let curr = self
            .todos
            .get_mut(self.list_state.selected().unwrap())
            .unwrap();
        curr.completed = !curr.completed;
    }

    pub fn delete_current_todo(&mut self) {
        let curr_idx = self.list_state.selected().unwrap();
        self.todos.remove(curr_idx);

        if self.todos.is_empty() {
            self.list_state.select(None);
        } else if curr_idx >= self.todos.len() {
            self.list_state.select(Some(self.todos.len() - 1));
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let list_items: Vec<ListItem> = self
            .todos
            .iter()
            .map(|todo| ListItem::new(App::format_todo(todo)))
            .collect();
        let title = match self.mode {
            Mode::Normal => "NORMAL",
            Mode::Insert => "INSERT"
        };
        let block = Block::bordered().title(title);
        let list = List::new(list_items).highlight_style(Style::new().reversed()).block(block);
        frame.render_stateful_widget(list, frame.area(), &mut self.list_state);
    }
}
