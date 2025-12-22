use terminal::{
    code::TerminalCode,
    elements::{Dispatch, TextLine},
    event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
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
}
impl Block for RrInput {
    fn pos(&self) -> (u16, u16, u16) {
        self.input.pos()
    }

    fn rel_line(&self, i: u16) -> Option<String> {
        self.input.rel_line(i)
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
            k @ TerminalCode::UnhandledKey(back_space!()) => {
                if self.hangul_result.read().unwrap().is_empty() {
                    k
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
