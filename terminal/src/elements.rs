use std::ops::Range;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::{
    code::TerminalCode,
    ext::{range_with_mid, saturate_range},
    traits::{Block, Input},
};
#[derive(Debug, Default, Clone)]
pub struct TextLine {
    pos: (u16, u16, u16),
    display_width: u16,
    index: u16,
    value: String,
}
impl TextLine {
    pub fn with_pos(&mut self, x: u16, y: u16) -> &mut Self {
        self.pos.0 = x;
        self.pos.1 = y;
        self
    }

    pub fn with_z_index(&mut self, z: u16) -> &mut Self {
        self.pos.2 = z;
        self
    }

    pub fn with_width(&mut self, width: u16) -> &mut Self {
        self.display_width = width;
        self
    }

    pub fn with_index(&mut self, index: u16) -> &mut Self {
        self.index = index;
        self
    }

    pub fn with_value(&mut self, value: String) -> &mut Self {
        self.display_width = self.display_width.max(1);
        self.value = value;
        self.index = self.len();
        self
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

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl Block for TextLine {
    fn pos(&self) -> (u16, u16, u16) {
        self.pos
    }

    fn rel_line(&self, i: u16) -> Option<String> {
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

impl Input for TextLine {
    fn feed(&mut self, key: KeyEvent) -> TerminalCode {
        match key {
            KeyEvent {
                code: KeyCode::Char(c),
                kind: KeyEventKind::Press,
                ..
            } => {
                self.value.insert(self.index as usize, c);
                self.index += 1;
                TerminalCode::None
            }
            KeyEvent {
                code: KeyCode::Left,
                kind: KeyEventKind::Press,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                self.index = self.index.saturating_sub_signed(1);
                TerminalCode::None
            }
            KeyEvent {
                code: KeyCode::Right,
                kind: KeyEventKind::Press,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                self.index += 1;
                self.index = self.index.min(self.len());
                TerminalCode::None
            }
            KeyEvent {
                code: KeyCode::Backspace,
                kind: KeyEventKind::Press,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                if self.len() == 0 || self.index == 0 {
                    TerminalCode::UnhandledKey(key)
                } else {
                    self.index -= 1;
                    self.value.remove(self.index as usize);
                    TerminalCode::None
                }
            }
            _ => TerminalCode::UnhandledKey(key),
        }
    }

    fn input_pos(&self) -> (u16, u16) {
        (self.pos().0, self.pos().1)
    }

    fn rel_cursor_pos(&self) -> Option<(u16, u16)> {
        self.display_range()
            .chain(Some(self.len() as usize))
            .enumerate()
            .find(|(_, el)| *el == self.index as usize)
            .map(|(i, _)| (i as u16, 0))
    }
}

#[derive(Debug, Clone, Default)]
pub struct LineVertical {
    pos: (u16, u16, u16),
    length: u16,
}
impl LineVertical {
    pub fn with_x(&mut self, x: u16) -> &mut Self {
        self.pos.0 = x;
        self
    }
    pub fn with_line_start(&mut self, y: u16) -> &mut Self {
        self.pos.1 = y;
        self
    }
    pub fn with_z_index(&mut self, z: u16) -> &mut Self {
        self.pos.2 = z;
        self
    }
    pub fn with_length(&mut self, length: u16) -> &mut Self {
        self.length = length;
        self
    }
}
impl Block for LineVertical {
    fn pos(&self) -> (u16, u16, u16) {
        self.pos
    }

    fn rel_line(&self, i: u16) -> Option<String> {
        if self.length == 0 {
            None
        } else if i == 0 || i == self.length - 1 {
            Some('+'.into())
        } else if i < self.length {
            Some('│'.into())
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct LineHorizontal {
    pos: (u16, u16, u16),
    length: u16,
}
impl LineHorizontal {
    pub fn with_y(&mut self, y: u16) -> &mut Self {
        self.pos.1 = y;
        self
    }
    pub fn with_line_start(&mut self, x: u16) -> &mut Self {
        self.pos.0 = x;
        self
    }
    pub fn with_z_index(&mut self, z: u16) -> &mut Self {
        self.pos.2 = z;
        self
    }
    pub fn with_length(&mut self, length: u16) -> &mut Self {
        self.length = length;
        self
    }
}
impl Block for LineHorizontal {
    fn pos(&self) -> (u16, u16, u16) {
        self.pos
    }

    fn rel_line(&self, i: u16) -> Option<String> {
        if self.length == 0 {
            None
        } else if self.length == 1 {
            Some('+'.into())
        } else if i == 0 {
            let mut line: String = "+".into();
            for _ in 1..(self.length - 1) {
                line.push('―');
            }
            line.push('+');
            Some(line)
        } else {
            None
        }
    }
}
