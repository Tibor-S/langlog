use std::ops::Range;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::{
    code::TerminalCode,
    ext::{IntoFork, range_with_mid, saturate_range},
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
        self.index = self.char_count() as u16;
        self
    }

    pub fn clear(&mut self) {
        self.index = 0;
        self.value = String::new()
    }

    pub fn display_range(&self) -> Range<usize> {
        let len = self.char_count().min(self.display_width as usize);
        saturate_range(
            range_with_mid(self.index as isize, len as isize),
            0..self.char_count() as usize,
        )
    }

    pub fn prefix_overflow(&self) -> bool {
        self.display_range().start > 0
    }

    pub fn char_count(&self) -> usize {
        self.value.chars().count()
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
         *                         012345678901234567890123
         *                                   1         2
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
        let display_range = self.display_range();
        let display = self.value[display_range.clone()].chars();
        let display = display.clone().fork_if(
            display_range.start != 0,
            Some('…').into_iter().chain(display.skip(1)),
        );
        let display = display.clone().fork_if(
            display_range.end != self.char_count(),
            display
                .take(display_range.len().saturating_sub(1))
                .chain(Some('…')),
        );
        Some(display.collect())
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
                self.index = self.index.min(self.char_count() as u16);
                TerminalCode::None
            }
            KeyEvent {
                code: KeyCode::Backspace,
                kind: KeyEventKind::Press,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                if self.char_count() == 0 || self.index == 0 {
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
            .chain(Some(self.char_count() as usize))
            .enumerate()
            .find(|(_, el)| *el == self.index as usize)
            .map(|(i, _)| (i as u16, 0))
    }
}
