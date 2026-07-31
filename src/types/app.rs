use ratatui::{Frame, widgets::ListState};

use crate::types::todo::Todo;

pub struct App {
    pub running: bool,
    pub todos: Vec<Todo>,
    pub list_state: ListState,
}

impl App {
    pub fn render(&mut self, frame: &mut Frame) {
        frame.render_widget("hello world", frame.area());
    }
}
