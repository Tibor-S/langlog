use std::{cmp::Ordering, env, io, ops::Range, path::PathBuf};

use csv::{ReaderBuilder, WriterBuilder};
use serde::{Deserialize, Serialize};
use terminal::{
    code::TerminalCode,
    elements::TextLine,
    event::{KeyCode, KeyEvent, KeyEventKind},
    ext::{range_with_mid, saturate_range},
    style::{Color, ContentStyle},
    traits::{Block, Input},
};

use crate::{ext::OrderedMap, hangul::Hangul};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Row {
    hangul: Hangul,
    description: String,
}
#[derive(Debug, Clone)]
pub struct Log {
    pos: (u16, u16, u16),
    input_pos: (u16, u16),
    width: u16,
    height: u16,
    entries: OrderedMap<Hangul, TextLine>,
    index: usize,
    focused: bool,
}
impl Log {
    pub const ENTRY_HEIGHT: usize = 3; // Hangul \ Description \ Br

    pub fn new(
        pos: (u16, u16, u16),
        width: u16,
        height: u16,
    ) -> io::Result<Self> {
        let entries: OrderedMap<Hangul, TextLine> =
            match Self::get_csv_records() {
                Ok(r) => r
                    .into_iter()
                    .map(|row| {
                        (
                            row.hangul,
                            Self::new_description(width, row.description),
                        )
                    })
                    .collect(),
                Err(e) => {
                    log::warn!("{}", e);
                    Default::default()
                }
            };
        Ok(Self {
            pos,
            input_pos: (pos.0, pos.1),
            width,
            height: height.saturating_sub(2),
            index: 0,
            entries,
            focused: false,
        })
    }

    pub fn save(&self) -> io::Result<()> {
        Self::set_csv_records(self.entries.iter().map(|(h, d)| Row {
            hangul: h.clone(),
            description: d.value().to_string(),
        }))
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
            .insert(key, Self::new_description(self.width, description))
            .map(|(k, v)| (k, v.value().to_string()));
        match (replaced, ordering) {
            (None, Some(Ordering::Less)) => {
                self.index += 1;
                None
            }
            (ret, None) => {
                self.index = 0;
                ret
            }
            (ret, _) => ret,
        }
    }

    pub fn remove_entry(&mut self, key: &Hangul) {
        let current = self.current_entry().map(|c| c.0.clone());
        match (current, self.entries.remove(key)) {
            (Some(current), Some(_)) if current < *key => {
                self.index -= 1;
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
                self.index = i;
                true
            }
            None => false,
        }
    }

    pub fn current_entry(&self) -> Option<&(Hangul, TextLine)> {
        self.entries.get(self.index)
    }

    pub fn line_index(&self) -> usize {
        self.index * Self::ENTRY_HEIGHT
    }

    pub fn line_count(&self) -> usize {
        self.entries.len() * Self::ENTRY_HEIGHT
    }

    fn display_range(&self) -> Range<usize> {
        let index = self.line_index();
        let len = self.line_count().min(self.height as usize);
        saturate_range(
            range_with_mid(index as isize, len as isize),
            0..self.line_count() as usize,
        )
    }

    fn get_csv_records() -> io::Result<Vec<Row>> {
        let csv_path = Self::csv_path()?;
        let mut rdr =
            ReaderBuilder::new().delimiter(b';').from_path(csv_path)?;

        Ok(rdr
            .deserialize()
            .filter_map(|res| {
                log::debug!("res: {:?}", res);
                match res {
                    Ok(v) => Some(v),
                    Err(e) => {
                        log::error!("{}", e);
                        None
                    }
                }
            })
            .collect::<Vec<_>>())
    }

    fn set_csv_records(
        entries: impl IntoIterator<Item = Row>,
    ) -> io::Result<()> {
        let csv_path = Self::csv_path()?;
        let mut wtr = WriterBuilder::new()
            .delimiter(b';')
            .has_headers(true)
            .from_path(csv_path)?;
        for entry in entries {
            wtr.serialize(entry)?;
        }
        Ok(())
    }

    fn csv_path() -> io::Result<PathBuf> {
        let mut csv_file = env::current_dir()?.into_os_string();
        csv_file.push("/hangul-log.csv");
        Ok(csv_file.into())
    }

    fn new_description(width: u16, text: String) -> TextLine {
        TextLine::default()
            .with_width(width)
            .with_value(text)
            .clone()
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
        let foc = if self.focused {
            self.index
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
                self.index = self.index.saturating_sub(1);
                TerminalCode::None
            }
            down!() => {
                self.index = (self.index + 1).min(self.entries.len() - 1);
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
