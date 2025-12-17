use std::io::{self, Stdout, Write};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    queue, style, terminal,
};

const END: usize = 18 * 2 + 1;
pub struct Border {}
impl Border {
    const LINES: [&'static str; 2] = [
        //   1    2    3    4    5    6    7    8    9    10   11   12   13   14   15   16   17   18   19   20
        "+――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――+",
        "|                                                                                                  |",
    ];
    const TOP_BOTTOM: &str = Self::LINES[0];
    const PIPE_ROW: &str = Self::LINES[1];
    fn line(i: usize) -> &'static str {
        const LAST: usize = END - 1;
        match i {
            0 | LAST => Self::TOP_BOTTOM,
            _ => Self::PIPE_ROW,
        }
    }
}

#[derive(Debug)]
pub struct Terminal {
    stdout: Stdout,
}
impl Terminal {
    pub fn new() -> Self {
        Self {
            stdout: io::stdout(),
        }
    }

    pub fn run(&mut self) -> TerminalResult<()> {
        crossterm::execute!(self.stdout, terminal::EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;

        loop {
            queue!(
                self.stdout,
                style::ResetColor,
                terminal::Clear(terminal::ClearType::All),
                cursor::MoveTo(0, 0)
            )?;
            for i in 0..END {
                queue!(
                    self.stdout,
                    style::Print(Border::line(i)),
                    cursor::MoveToNextLine(1)
                )?;
            }
            self.stdout.flush()?;
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

#[derive(Debug, thiserror::Error)]
pub enum TerminalError {
    #[error(transparent)]
    IO(#[from] io::Error),
}
pub type TerminalResult<T> = Result<T, TerminalError>;
