use std::ops::Range;

use crossterm::{event::KeyEvent, style::ContentStyle};

use crate::code::TerminalCode;

pub trait Block {
    fn pos(&self) -> (u16, u16, u16);
    fn rel_line(&self, i: u16) -> Option<String>;
    /// Unspecified ranges will be printed without style
    fn style_line(&self, i: u16) -> Vec<(Range<usize>, ContentStyle)> {
        let _ = i;
        vec![]
    }
}
impl Block for () {
    fn pos(&self) -> (u16, u16, u16) {
        Default::default()
    }

    fn rel_line(&self, i: u16) -> Option<String> {
        let _ = i;
        Default::default()
    }
}

pub trait Input: Block {
    /// Unavailable KeyEvents:
    /// - `KeyModifiers::CONTROL + KeyCode::Char('q')`
    /// - `KeyCode::Tab`
    /// - `KeyCode::BackTab`
    fn feed(&mut self, key: KeyEvent) -> TerminalCode;
    /// None if cursor is not shown
    fn rel_cursor_pos(&self) -> Option<(u16, u16)>;
    fn input_pos(&self) -> (u16, u16);
    /// Called when element is focused
    fn focus(&mut self) {}
    /// Called when element is unfocused
    fn unfocus(&mut self) {}
}
impl Input for () {
    fn feed(&mut self, key: KeyEvent) -> TerminalCode {
        TerminalCode::UnhandledKey(key)
    }

    fn rel_cursor_pos(&self) -> Option<(u16, u16)> {
        Default::default()
    }

    fn input_pos(&self) -> (u16, u16) {
        Default::default()
    }
}
