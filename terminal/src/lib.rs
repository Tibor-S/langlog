mod ext;

use std::{
    fmt::Display,
    io::{self, Stdout, Write},
    ops::Range,
};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    queue, style, terminal,
};

use crate::ext::{range_with_mid, saturate_range};

pub trait Block {
    type Line: Display;

    fn pos(&self) -> (u16, u16);
    fn rel_line(&self, i: u16) -> Option<Self::Line>;
}

pub trait Input {
    type Value;

    fn feed(&mut self, key: KeyEvent) -> Option<KeyEvent>;
    /// None if cursor is not shown
    fn rel_cursor_pos(&self) -> Option<(u16, u16)>;
    fn value(&self) -> &Self::Value;
}

#[derive(Debug, Default, Clone)]
pub struct TextInput {
    pos: (u16, u16),
    display_width: u16,
    index: u16,
    value: String,
}

impl TextInput {
    pub fn with_pos(self, x: u16, y: u16) -> Self {
        Self {
            pos: (x, y),
            ..self
        }
    }

    pub fn with_width(self, width: u16) -> Self {
        Self {
            display_width: width,
            ..self
        }
    }

    pub fn display_range(&self) -> Range<usize> {
        let len = self.len().min(self.display_width);
        saturate_range(
            range_with_mid(self.index as isize, len as isize),
            0..self.len() as usize,
        )
    }

    pub fn len(&self) -> u16 {
        self.value.chars().count() as u16
    }
}

impl Block for TextInput {
    type Line = String;

    fn pos(&self) -> (u16, u16) {
        self.pos
    }

    fn rel_line(&self, i: u16) -> Option<Self::Line> {
        if i != 0 {
            return None;
        }
        /* EX:
         * display_width == 16
         * len() == 24 && value = "abcdefghijklmnopqrstuvwx"
         *                               0123456789ABCDEFGHIJKLMN
         * self.index = 3
         * "abcdefghijklmno…"
         *     ^
         * self.index = 12
         * "…ghijklmnopqrst…"
         *         ^
         * self.index = 17
         * "…jklmnopqrstuvwx"
         *           ^
         */
        let mut display_range = self.display_range();
        if self.len() <= self.display_width {
            Some(self.value.clone())
        } else if display_range.start == 0 {
            display_range.end -= 1;
            let mut ret = String::from(&self.value[display_range]);
            ret.push('…');
            Some(ret)
        } else if display_range.end == self.len() as usize {
            display_range.start += 1;
            let mut ret = String::from('…');
            ret.extend(self.value[display_range].chars());
            Some(ret)
        } else {
            display_range.start += 1;
            display_range.end -= 1;
            let mut ret = String::from('…');
            ret.extend(self.value[display_range].chars());
            ret.push('…');
            Some(ret)
        }
    }
}

impl Input for TextInput {
    type Value = String;

    fn feed(&mut self, key: KeyEvent) -> Option<KeyEvent> {
        match key {
            KeyEvent {
                code: KeyCode::Char(c),
                kind: KeyEventKind::Press,
                modifiers,
                ..
            } if modifiers != KeyModifiers::CONTROL => {
                self.value.insert(self.index as usize, c);
                self.index += 1;
                None
            }
            KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                ..
            } => {
                self.index = self.index.saturating_sub_signed(1);
                None
            }
            KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                ..
            } => {
                self.index += 1;
                self.index = self.index.min(self.len());
                None
            }
            KeyEvent {
                code: KeyCode::Backspace,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                ..
            } => {
                if self.len() == 0 {
                    Some(key)
                } else {
                    self.index -= 1;
                    self.value.remove(self.index as usize);
                    None
                }
            }
            _ => Some(key),
        }
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }

    fn rel_cursor_pos(&self) -> Option<(u16, u16)> {
        self.display_range()
            .chain(Some(self.len() as usize))
            .enumerate()
            .find(|(_, el)| *el == self.index as usize)
            .map(|(i, _)| (i as u16, 0))
    }
}

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
        let mut inp = TextInput::default().with_pos(5, 5).with_width(24);

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
            self.draw_block(&inp)?;
            self.input_cursor(&inp)?;
            self.stdout.flush()?;
            match Self::read_key()? {
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: KeyEventKind::Press,
                    ..
                } => {
                    // execute!(w, cursor::SetCursorStyle::DefaultUserShape).unwrap();
                    break;
                }
                event => {
                    inp.feed(event);
                }
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

    fn draw_block(&mut self, block: &impl Block) -> io::Result<()> {
        let (x, y) = block.pos();
        queue!(self.stdout, cursor::MoveTo(x, y))?;
        let mut i = 0;
        while let Some(line) = block.rel_line(i) {
            i += 1;
            queue!(self.stdout, style::Print(line), cursor::MoveTo(x, y + i))?;
        }

        Ok(())
    }

    fn input_cursor<Inp>(&mut self, input: &Inp) -> io::Result<()>
    where
        Inp: Block + Input,
    {
        if let Some((rx, ry)) = input.rel_cursor_pos() {
            let (x, y) = input.pos();
            queue!(self.stdout, cursor::Show, cursor::MoveTo(x + rx, y + ry))
        } else {
            self.hide_cursor()
        }
    }

    fn hide_cursor(&mut self) -> io::Result<()> {
        queue!(self.stdout, cursor::Hide)
    }

    fn read_key() -> TerminalResult<KeyEvent> {
        loop {
            if let Ok(Event::Key(event)) = event::read() {
                return Ok(event);
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
