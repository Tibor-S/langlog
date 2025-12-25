use crossterm::event::KeyEvent;

#[derive(Debug, Clone)]
pub enum TerminalCode {
    None,
    Exit,
    PreviousScene,
    PreviousSceneWithFocus(usize),
    GoToScene(String),
    ReplaceCurrentScene(String),
    Focus(usize),
    FocusAt((u16, u16)),
    UnhandledKey(KeyEvent),
}
