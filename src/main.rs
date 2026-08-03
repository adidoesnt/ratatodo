pub mod types;

use crossterm::event::{Event, KeyCode};
use ratatui::DefaultTerminal;
use tokio::runtime::Runtime;

use crate::types::{app::{App, Mode, NavigationType}, repository::Repository};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let runtime = Runtime::new()?;
    let handle = runtime.handle().clone();
    let mut app = runtime.block_on(async {
        let repo = Repository::new().await?;
        App::new(repo, handle).await
    })?;

    ratatui::run(|terminal| run_app(terminal, &mut app))?;
    Ok(())
}

fn run_app(terminal: &mut DefaultTerminal, app: &mut App) -> std::io::Result<()> {
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
                    KeyCode::Char(c) => {
                        app.user_input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.user_input.pop();
                    }
                    KeyCode::Enter => {
                        app.add_todo_from_user_input();
                    }
                    KeyCode::Esc => {
                        app.user_input = String::new();
                        app.set_mode(Mode::Normal);
                    }
                    _ => {}
                },
            }
        }
    }

    Ok(())
}
