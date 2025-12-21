use crossterm::event::KeyEvent;

pub enum TerminalCode {
    None,
    Exit,
    UnhandledKey(KeyEvent),
}
