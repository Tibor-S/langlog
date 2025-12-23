use crossterm::event::KeyEvent;

pub enum TerminalCode {
    None,
    Exit,
    Focus(usize),
    FocusAt((u16, u16)),
    UnhandledKey(KeyEvent),
}
