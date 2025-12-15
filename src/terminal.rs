use std::io::{self, Stdout};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    style, terminal,
};

pub struct Terminal {
    stdout: Stdout,
}
impl Terminal {
    pub fn run(&mut self) -> TerminalResult<()> {
        crossterm::execute!(self.stdout, terminal::EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;

        loop {
            match Self::read_char()? {
                'q' => {
                    // execute!(w, cursor::SetCursorStyle::DefaultUserShape).unwrap();
                    break;
                }
                _ => {}
            };
        }

        crossterm::execute!(
            self.stdout,
            style::ResetColor,
            cursor::Show,
            terminal::LeaveAlternateScreen
        )?;

        terminal::disable_raw_mode()?;
        Ok(())
    }

    fn read_char() -> TerminalResult<char> {
        loop {
            if let Ok(Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                kind: KeyEventKind::Press,
                modifiers: _,
                state: _,
            })) = event::read()
            {
                return Ok(c);
            }
        }
    }
}
impl Default for Terminal {
    fn default() -> Self {
        Self {
            stdout: io::stdout(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TerminalError {
    #[error(transparent)]
    IO(#[from] io::Error),
}
pub type TerminalResult<T> = Result<T, TerminalError>;
