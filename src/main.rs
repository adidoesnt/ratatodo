pub mod mocks;
pub mod types;

use crossterm::event::{Event, KeyCode};
use ratatui::{DefaultTerminal, widgets::ListState};

use crate::types::app::{App, Mode, NavigationType};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let list_state = ListState::default().with_selected(Some(0));
    let mut app = App {
        running: true,
        mode: Mode::Normal,
        todos: mocks::todo::mock_todos(),
        list_state,
    };

    while app.running {
        terminal.draw(|frame| app.render(frame))?;
        if let Event::Key(key) = crossterm::event::read()? {
            // Skip events that are not KeyEventKind::Press
            if key.kind == crossterm::event::KeyEventKind::Release {
                continue;
            }

            match app.mode {
                Mode::Normal => match key.code {
                    KeyCode::Char('q') => {
                        app.running = false;
                    }
                    KeyCode::Down => {
                        app.navigate(NavigationType::Increment);
                    }
                    KeyCode::Up => {
                        app.navigate(NavigationType::Decrement);
                    }
                    KeyCode::Char(' ') => {
                        app.toggle_current_todo();
                    }
                    KeyCode::Char('d') => {
                        app.delete_current_todo();
                    }
                    KeyCode::Char('i') => {
                        app.set_mode(Mode::Insert);
                    }
                    _ => {}
                },
                Mode::Insert => match key.code {
                    KeyCode::Esc => {
                        app.set_mode(Mode::Normal);
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
