use std::ops::Range;

use terminal::{
    code::TerminalCode,
    elements::{Dispatch, TextLine},
    event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    style::{Attribute, Attributes, Color, ContentStyle},
    traits::{Block, Input},
};

use crate::elements::HangulResult;

macro_rules! enter {
    () => {
        KeyEvent {
            code: KeyCode::Enter,
            kind: KeyEventKind::Press,
            modifiers: KeyModifiers::NONE,
            ..
        }
    };
}
macro_rules! back_space {
    () => {
        KeyEvent {
            code: KeyCode::Backspace,
            kind: KeyEventKind::Press,
            modifiers: KeyModifiers::NONE,
            ..
        }
    };
}

#[derive(Debug, Default, Clone)]
pub struct RrInput {
    input: TextLine,
    hangul_result: Dispatch<HangulResult>,
}
impl RrInput {
    pub fn new(input: TextLine, hangul_result: Dispatch<HangulResult>) -> Self {
        Self {
            input,
            hangul_result,
        }
    }

    pub fn clear(&mut self) {
        self.input.clear();
        self.hangul_result.write().unwrap().clear()
    }

    pub fn hangul(&self) -> Dispatch<HangulResult> {
        self.hangul_result.clone()
    }
}
impl Block for RrInput {
    fn pos(&self) -> (u16, u16, u16) {
        self.input.pos()
    }

    fn rel_line(&self, i: u16) -> Option<String> {
        self.input.rel_line(i)
    }

    fn style_line(&self, i: u16) -> Vec<(Range<usize>, ContentStyle)> {
        if i != 0 {
            return vec![];
        }
        let style = ContentStyle {
            foreground_color: Some(Color::Red),
            underline_color: Some(Color::Red),
            attributes: Attributes::none()
                .with(Attribute::Underlined)
                .with(Attribute::NoBlink)
                .with(Attribute::NotCrossedOut),
            ..Default::default()
        };
        if self.input.prefix_overflow() {
            return vec![(0..usize::MAX, style)];
        }
        let mut error_range =
            0..self.hangul_result.read().unwrap().overflow().len();
        let diff = (self.input.char_count() as usize)
            .saturating_sub(error_range.len());
        error_range.start += diff;
        error_range.end += diff;

        vec![(error_range, style)]
    }
}
impl Input for RrInput {
    fn feed(
        &mut self,
        key: terminal::event::KeyEvent,
    ) -> terminal::code::TerminalCode {
        match self.input.feed(key) {
            TerminalCode::None => {
                self.hangul_result
                    .write()
                    .unwrap()
                    .set_rr(self.input.value());
                TerminalCode::None
            }
            TerminalCode::UnhandledKey(enter!()) => {
                self.hangul_result.write().unwrap().push();
                self.input.with_value(
                    self.hangul_result.read().unwrap().overflow().clone(),
                );
                self.hangul_result
                    .write()
                    .unwrap()
                    .set_rr(self.input.value());
                TerminalCode::None
            }
            c @ TerminalCode::UnhandledKey(back_space!()) => {
                if self.hangul_result.read().unwrap().is_empty() {
                    c
                } else {
                    self.hangul_result.write().unwrap().pop();
                    TerminalCode::None
                }
            }
            c => c,
        }
    }

    fn rel_cursor_pos(&self) -> Option<(u16, u16)> {
        self.input.rel_cursor_pos()
    }

    fn input_pos(&self) -> (u16, u16) {
        self.input.input_pos()
    }
}
