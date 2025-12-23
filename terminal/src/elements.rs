use std::{
    fmt,
    iter::repeat,
    ops::{Deref, DerefMut, Range},
    rc::Rc,
    sync::RwLock,
};

use crossterm::{
    event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    style::{Color, ContentStyle},
};

use crate::{
    code::TerminalCode,
    ext::{IntoFork, range_with_mid, saturate_range},
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

#[derive(Debug, Default)]
pub struct Dispatch<T>(Rc<RwLock<T>>);
impl<T> From<T> for Dispatch<T> {
    fn from(value: T) -> Self {
        Self(Rc::new(RwLock::new(value)))
    }
}
impl<T: Clone> From<&T> for Dispatch<T> {
    fn from(value: &T) -> Self {
        Self(Rc::new(RwLock::new(value.clone())))
    }
}
impl<T: Clone> From<&mut T> for Dispatch<T> {
    fn from(value: &mut T) -> Self {
        Self(Rc::new(RwLock::new(value.clone())))
    }
}
impl<T> Clone for Dispatch<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}
impl<T> Deref for Dispatch<T> {
    type Target = Rc<RwLock<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for Dispatch<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<B: Block> Block for Dispatch<B> {
    fn pos(&self) -> (u16, u16, u16) {
        self.read().unwrap().pos()
    }

    fn rel_line(&self, i: u16) -> Option<String> {
        self.read().unwrap().rel_line(i)
    }

    fn style_line(&self, i: u16) -> Vec<(Range<usize>, ContentStyle)> {
        self.read().unwrap().style_line(i)
    }
}
impl<I: Input> Input for Dispatch<I> {
    fn feed(&mut self, key: KeyEvent) -> TerminalCode {
        self.write().unwrap().feed(key)
    }

    fn rel_cursor_pos(&self) -> Option<(u16, u16)> {
        self.read().unwrap().rel_cursor_pos()
    }

    fn input_pos(&self) -> (u16, u16) {
        self.read().unwrap().input_pos()
    }

    fn focus(&mut self) {
        self.0.write().unwrap().focus();
    }

    fn unfocus(&mut self) {
        self.0.write().unwrap().unfocus();
    }
}

#[derive(Debug, Clone, Default)]
pub struct LineVertical {
    pos: (u16, u16, u16),
    length: u16,
}
impl LineVertical {
    pub fn with_x(&mut self, x: u16) -> &mut Self {
        self.pos.0 = x;
        self
    }
    pub fn with_line_start(&mut self, y: u16) -> &mut Self {
        self.pos.1 = y;
        self
    }
    pub fn with_z_index(&mut self, z: u16) -> &mut Self {
        self.pos.2 = z;
        self
    }
    pub fn with_length(&mut self, length: u16) -> &mut Self {
        self.length = length;
        self
    }
}
impl Block for LineVertical {
    fn pos(&self) -> (u16, u16, u16) {
        self.pos
    }

    fn rel_line(&self, i: u16) -> Option<String> {
        if self.length == 0 {
            None
        } else if i == 0 || i == self.length - 1 {
            Some('+'.into())
        } else if i < self.length {
            Some('│'.into())
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct LineHorizontal {
    pos: (u16, u16, u16),
    length: u16,
}
impl LineHorizontal {
    pub fn with_y(&mut self, y: u16) -> &mut Self {
        self.pos.1 = y;
        self
    }
    pub fn with_line_start(&mut self, x: u16) -> &mut Self {
        self.pos.0 = x;
        self
    }
    pub fn with_z_index(&mut self, z: u16) -> &mut Self {
        self.pos.2 = z;
        self
    }
    pub fn with_length(&mut self, length: u16) -> &mut Self {
        self.length = length;
        self
    }
}
impl Block for LineHorizontal {
    fn pos(&self) -> (u16, u16, u16) {
        self.pos
    }

    fn rel_line(&self, i: u16) -> Option<String> {
        if self.length == 0 {
            None
        } else if self.length == 1 {
            Some('+'.into())
        } else if i == 0 {
            let mut line: String = "+".into();
            for _ in 1..(self.length - 1) {
                line.push('―');
            }
            line.push('+');
            Some(line)
        } else {
            None
        }
    }
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
