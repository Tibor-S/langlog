use crossterm::event::KeyEvent;

use crate::code::TerminalCode;

pub trait Block {
    fn pos(&self) -> (u16, u16, u16);
    fn rel_line(&self, i: u16) -> Option<String>;
}

pub trait Input: Block {
    /// Unavailable KeyEvents:
    /// - `KeyModifiers::CONTROL + KeyCode::Char('q')`
    /// - `KeyCode::Up`
    /// - `KeyCode::DOWN`
    fn feed(&mut self, key: KeyEvent) -> TerminalCode;
    /// None if cursor is not shown
    fn rel_cursor_pos(&self) -> Option<(u16, u16)>;
    fn input_pos(&self) -> (u16, u16);
    /// Called when element is focused
    fn focus(&mut self) {}
    /// Called when element is unfocused
    fn unfocus(&mut self) {}
}
