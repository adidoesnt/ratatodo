use ratatui::{
    Frame,
    style::Style,
    widgets::{Block, List, ListItem, ListState},
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
    pub user_input: String,
}

impl App {
    fn format_todo(todo: &Todo, index: usize) -> String {
        let checkbox = match todo.completed {
            true => "[x]",
            false => "[ ]",
        };
        let title = &todo.title;

        format!("{checkbox} | {index}) {title}")
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
        if let Some(idx) = self.list_state.selected()
            && let Some(curr) = self.todos.get_mut(idx)
        {
            curr.completed = !curr.completed;
        }
    }

    pub fn delete_current_todo(&mut self) {
        if let Some(curr_idx) = self.list_state.selected() {
            self.todos.remove(curr_idx);

            if self.todos.is_empty() {
                self.list_state.select(None);
            } else if curr_idx >= self.todos.len() {
                self.list_state.select(Some(self.todos.len() - 1));
            }
        }
    }

    pub fn add_todo_from_user_input(&mut self) {
        match self.mode {
            Mode::Insert => {
                let user_input = self.user_input.trim().to_string();
                if !user_input.is_empty() {
                    let todo = Todo {
                        title: user_input,
                        completed: false,
                    };
                    self.todos.push(todo);
                    self.mode = Mode::Normal;
                    self.user_input = String::new();
                }
            }
            Mode::Normal => {}
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let list_items: Vec<ListItem> = self
            .todos
            .iter()
            .enumerate()
            .map(|(idx, todo)| ListItem::new(App::format_todo(todo, idx)))
            .collect();
        let title = match self.mode {
            Mode::Normal => "NORMAL".to_string(),
            Mode::Insert => format!("INSERT: {} ", self.user_input),
        };
        let block = Block::bordered().title(title);
        let list = List::new(list_items)
            .highlight_style(Style::new().reversed())
            .block(block);
        frame.render_stateful_widget(list, frame.area(), &mut self.list_state);
    }
}
