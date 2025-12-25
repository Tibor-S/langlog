use crossterm::event::KeyEvent;

#[derive(Debug, Clone)]
pub enum TerminalCode {
    None,
    Exit,
    PreviousScene,
    GoToScene(String),
    Focus(usize),
    FocusAt((u16, u16)),
    UnhandledKey(KeyEvent),
}
