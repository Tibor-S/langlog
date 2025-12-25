use std::{cmp::Ordering, ops::Range};

use terminal::{
    code::TerminalCode,
    elements::TextLine,
    event::{KeyCode, KeyEvent, KeyEventKind},
    ext::{range_with_mid, saturate_range},
    style::{Color, ContentStyle},
    traits::{Block, Input},
};

use crate::{ext::OrderedMap, hangul::Hangul};

#[derive(Debug, Clone)]
pub struct Log {
    pos: (u16, u16, u16),
    input_pos: (u16, u16),
    width: u16,
    height: u16,
    entries: OrderedMap<Hangul, TextLine>,
    index: Option<usize>,
    focused: bool,
}
impl Log {
    pub const ENTRY_HEIGHT: usize = 3; // Hangul \ Description \ Br

    pub fn new(pos: (u16, u16, u16), width: u16, height: u16) -> Self {
        Self {
            pos,
            input_pos: (pos.0, pos.1),
            width,
            height: height.saturating_sub(2),
            index: None,
            entries: Default::default(),
            focused: false,
        }
    }

    pub fn with_input_pos(&mut self, pos: (u16, u16)) -> &mut Self {
        self.input_pos = pos;
        self
    }

    pub fn insert_entry(
        &mut self,
        key: Hangul,
        description: String,
    ) -> Option<(Hangul, String)> {
        let ordering = self.current_entry().map(|(k, _)| key.cmp(k));
        let replaced = self
            .entries
            .insert(
                key,
                TextLine::default()
                    .with_width(self.width)
                    .with_value(description)
                    .clone(),
            )
            .map(|(k, v)| (k, v.value().to_string()));
        match (replaced, ordering) {
            (None, Some(Ordering::Less)) => {
                self.index.as_mut().map(|i| *i += 1);
                None
            }
            (ret, None) => {
                self.index = Some(0);
                ret
            }
            (ret, _) => ret,
        }
    }

    pub fn remove_entry(&mut self, key: &Hangul) {
        let current = self.current_entry().map(|c| c.0.clone());
        match (current, self.entries.remove(key)) {
            (Some(current), Some(_)) if current < *key => {
                self.index.as_mut().map(|i| *i -= 1);
            }
            _ => (),
        }
    }

    pub fn index_at(&mut self, key: &Hangul) -> bool {
        let found =
            self.entries.iter().enumerate().find_map(|(i, (k, _))| {
                if *k == *key { Some(i) } else { None }
            });
        match found {
            Some(i) => {
                self.index = Some(i);
                true
            }
            None => false,
        }
    }

    pub fn current_entry(&self) -> Option<&(Hangul, TextLine)> {
        self.index.map(|i| self.entries.get(i)).unwrap_or(None)
    }

    pub fn line_index(&self) -> Option<usize> {
        self.index.map(|i| i * Self::ENTRY_HEIGHT)
    }

    pub fn line_count(&self) -> usize {
        self.entries.len() * Self::ENTRY_HEIGHT
    }

    fn display_range(&self) -> Range<usize> {
        let index = match self.line_index() {
            Some(i) => i,
            None => return 0..0,
        };
        let len = self.line_count().min(self.height as usize);
        saturate_range(
            range_with_mid(index as isize, len as isize),
            0..self.line_count() as usize,
        )
    }
}
impl Block for Log {
    fn pos(&self) -> (u16, u16, u16) {
        self.pos
    }

    fn rel_line(&self, i: u16) -> Option<String> {
        // Header
        match i {
            0 => return Some("Log:".into()),
            1 => return Some("".into()),
            _ => (),
        }

        // Entries
        let i = (i as usize).saturating_sub(2);
        let display_range = self.display_range();
        if display_range.len() <= i {
            return None;
        }

        if i == 0 && display_range.start != 0 {
            Some('…'.into())
        } else if i == display_range.end - 1
            && display_range.end != self.line_count()
        {
            Some('…'.into())
        } else {
            let real_line = display_range.start + i;
            let entry_index = real_line / Self::ENTRY_HEIGHT;
            let entry_line = real_line % Self::ENTRY_HEIGHT;
            match entry_line {
                0 => {
                    self.entries.get(entry_index).map(|(h, _)| format!("{}", h))
                }
                1 => self
                    .entries
                    .get(entry_index)
                    .map(|(_, tl)| tl.rel_line(0))
                    .unwrap_or(None),
                2 => Some("".into()),
                _ => panic!("Logic Error!"),
            }
        }
    }

    fn style_line(&self, i: u16) -> Vec<(Range<usize>, ContentStyle)> {
        let foc = if self.focused
            && let Some(index) = self.index
        {
            index
        } else {
            return vec![];
        };
        let i = if i < 2 {
            return vec![];
        } else {
            (i as usize).saturating_sub(2)
        };
        let display_range = self.display_range();
        let real_line = display_range.start + i;
        let entry_index = real_line / Self::ENTRY_HEIGHT;

        if entry_index != foc {
            return vec![];
        }

        let entry_line = real_line % Self::ENTRY_HEIGHT;

        match entry_line {
            0 => vec![(
                0..usize::MAX,
                ContentStyle {
                    foreground_color: Some(Color::Black),
                    background_color: Some(Color::White),
                    ..Default::default()
                },
            )],
            _ => vec![],
        }
    }
}

macro_rules! up {
    () => {
        KeyEvent {
            code: KeyCode::Up,
            kind: KeyEventKind::Press,
            ..
        }
    };
}
macro_rules! down {
    () => {
        KeyEvent {
            code: KeyCode::Down,
            kind: KeyEventKind::Press,
            ..
        }
    };
}
impl Input for Log {
    fn feed(&mut self, key: KeyEvent) -> TerminalCode {
        match key {
            up!() => {
                self.index =
                    Some(self.index.map(|i| i.saturating_sub(1)).unwrap_or(0));
                TerminalCode::None
            }
            down!() => {
                self.index = Some(
                    self.index
                        .map(|i| (i + 1).min(self.entries.len() - 1))
                        .unwrap_or(0),
                );
                TerminalCode::None
            }
            _ => TerminalCode::UnhandledKey(key),
        }
    }

    fn rel_cursor_pos(&self) -> Option<(u16, u16)> {
        None
    }

    fn input_pos(&self) -> (u16, u16) {
        self.input_pos
    }

    fn focus(&mut self) {
        self.focused = true
    }

    fn unfocus(&mut self) {
        self.focused = false
    }
}
