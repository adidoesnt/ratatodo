use ratatui::{
    Frame,
    style::Style,
    widgets::{Block, List, ListItem, ListState},
};
use tokio::runtime::Handle;

use crate::types::{repository::Repository, todo::Todo};

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
    pub repository: Repository,
    pub handle: Handle,
}

impl App {
    pub async fn new(repo: Repository, handle: Handle) -> std::io::Result<Self> {
        let todos = repo.get_all_todos().await?;
        let list_state =
            ListState::default().with_selected(if todos.is_empty() { None } else { Some(0) });

        Ok(Self {
            running: true,
            mode: Mode::Normal,
            todos,
            list_state,
            user_input: String::new(),
            repository: repo,
            handle,
        })
    }

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
            if let Some(id) = curr.id {
                let _ = self
                    .handle
                    .block_on(self.repository.toggle_todo(id, curr.completed));
            }
        }
    }

    pub fn delete_current_todo(&mut self) {
        if let Some(curr_idx) = self.list_state.selected() {
            let todo = self.todos.remove(curr_idx);
            if let Some(id) = todo.id {
                let _ = self.handle.block_on(self.repository.delete_todo(id));
            }

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
                    if let Ok(todo) = self
                        .handle
                        .block_on(self.repository.add_todo(&user_input))
                    {
                        self.todos.push(todo);
                    }
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
