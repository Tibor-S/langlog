use std::{
    ops::{Deref, DerefMut, Range},
    rc::Rc,
    sync::RwLock,
};

use crossterm::{event::KeyEvent, style::ContentStyle};

use crate::{
    code::TerminalCode,
    traits::{Block, Input},
};

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
