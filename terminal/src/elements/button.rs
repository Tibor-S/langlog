use std::{fmt, iter::repeat, ops::Range};

use crossterm::{
    event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    style::{Color, ContentStyle},
};

use crate::{
    code::TerminalCode,
    elements::TextLine,
    traits::{Block, Input},
};

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

pub struct Button<F> {
    pos: (u16, u16, u16),
    text: TextLine,
    width: u16,
    margin: u16,
    on_enter: Option<F>,
    focused: bool,
}
impl<F> Button<F>
where
    F: Fn() -> TerminalCode,
{
    pub fn new(
        pos: (u16, u16, u16),
        text: String,
        width: u16,
        margin: u16,
        on_enter: Option<F>,
    ) -> Self {
        let text = TextLine::default()
            .with_width(width.saturating_sub(2 * margin))
            .with_value(text)
            .clone();
        Self {
            pos,
            text,
            width,
            margin,
            on_enter,
            focused: false,
        }
    }
}
impl<F> Block for Button<F> {
    fn pos(&self) -> (u16, u16, u16) {
        self.pos
    }

    fn rel_line(&self, i: u16) -> Option<String> {
        if i != 0 {
            return None;
        }
        Some(String::from_iter(
            repeat(' ')
                .take(self.margin as usize)
                .chain(self.text.rel_line(i).unwrap_or(String::new()).chars())
                .chain(repeat(' ').take(self.margin as usize)),
        ))
    }

    fn style_line(&self, i: u16) -> Vec<(Range<usize>, ContentStyle)> {
        let _ = i;
        let style = if self.focused {
            ContentStyle {
                foreground_color: Some(Color::Black),
                background_color: Some(Color::White),
                ..Default::default()
            }
        } else {
            ContentStyle {
                foreground_color: Some(Color::White),
                background_color: Some(Color::Black),
                ..Default::default()
            }
        };
        std::vec![(0..usize::MAX, style)]
    }
}
impl<F> Input for Button<F>
where
    F: Fn() -> TerminalCode,
{
    fn feed(&mut self, key: KeyEvent) -> TerminalCode {
        match key {
            enter!() => self
                .on_enter
                .as_ref()
                .map(|f| f())
                .unwrap_or(TerminalCode::None),
            k => TerminalCode::UnhandledKey(k),
        }
    }

    fn rel_cursor_pos(&self) -> Option<(u16, u16)> {
        None
    }

    fn input_pos(&self) -> (u16, u16) {
        (self.pos.0, self.pos.1)
    }

    fn focus(&mut self) {
        self.focused = true
    }

    fn unfocus(&mut self) {
        self.focused = false
    }
}
impl<F> fmt::Debug for Button<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Button")
            .field("pos", &self.pos)
            .field("text", &self.text)
            .field("width", &self.width)
            .field("margin", &self.margin)
            .finish()
    }
}
