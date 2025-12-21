use std::{rc::Rc, sync::RwLock};

use terminal::{
    code::TerminalCode,
    elements::TextLine,
    event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    traits::{Block, Input},
};

use crate::{
    hangul::Hangul, hangul_parser::HangulParser, jamo::Jamo, syllable::Syllable,
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
macro_rules! back_space {
    () => {
        KeyEvent {
            code: KeyCode::Backspace,
            kind: KeyEventKind::Press,
            modifiers: KeyModifiers::NONE,
            ..
        }
    };
}
#[derive(Debug, Default)]
pub struct HangulInput {
    pos: (u16, u16, u16),
    width: u16,
    syllable: Rc<RwLock<Syllable>>,
    overflow: String,
    input: TextLine,
    str: Hangul,
    parser: HangulParser,
}
impl HangulInput {
    pub fn new(pos: (u16, u16, u16), width: u16) -> Self {
        Self {
            pos,
            width,
            input: TextLine::default()
                .with_pos(pos.0, pos.1 + 2)
                .with_width(width)
                .clone(),
            ..Default::default()
        }
    }

    pub fn syllable(&self) -> Rc<RwLock<Syllable>> {
        Rc::clone(&self.syllable)
    }
}
impl Block for HangulInput {
    fn pos(&self) -> (u16, u16, u16) {
        self.pos
    }

    fn rel_line(&self, i: u16) -> Option<String> {
        match i {
            0 => Some(format!("{}{}", self.str, self.syllable.read().unwrap())),
            1 => Some("".into()),
            2 => self.input.rel_line(0),
            _ => None,
        }
    }
}
impl Input for HangulInput {
    fn feed(&mut self, key: KeyEvent) -> TerminalCode {
        match self.input.feed(key) {
            TerminalCode::None => {
                let overflow;
                (*self.syllable.write().unwrap(), overflow) =
                    self.parser.parse_syllable(self.input.value());
                self.overflow = overflow.into();
                TerminalCode::None
            }
            TerminalCode::UnhandledKey(enter!()) => {
                self.str.push(self.syllable.read().unwrap().clone());
                self.input.with_value(self.overflow.clone());
                let overflow;
                (*self.syllable.write().unwrap(), overflow) =
                    self.parser.parse_syllable(self.input.value());
                self.overflow = overflow.into();
                TerminalCode::None
            }
            k @ TerminalCode::UnhandledKey(back_space!()) => {
                if self.str.is_empty() {
                    k
                } else {
                    (*self.str).pop();
                    TerminalCode::None
                }
            }
            c => c,
        }
    }

    fn rel_cursor_pos(&self) -> Option<(u16, u16)> {
        self.input.rel_cursor_pos()
    }

    fn input_pos(&self) -> (u16, u16) {
        self.input.input_pos()
    }
}

pub struct PossibleInfo {
    pos: (u16, u16, u16),
    syllable: Rc<RwLock<Syllable>>,
}
impl PossibleInfo {
    pub fn new(pos: (u16, u16, u16), syllable: Rc<RwLock<Syllable>>) -> Self {
        Self { pos, syllable }
    }
}
impl Block for PossibleInfo {
    fn pos(&self) -> (u16, u16, u16) {
        self.pos
    }

    fn rel_line(&self, i: u16) -> Option<String> {
        match i {
            0 => Some("Combinations:".into()),
            1 => {
                let mut str: String = "".into();
                if let Some(possible) = self.syllable.read().unwrap().finale() {
                    for j in possible.append_possible() {
                        str.push(Jamo::from(j).into());
                        str.push_str("  ");
                    }
                    Some(str)
                } else if let Some(possible) =
                    self.syllable.read().unwrap().medial()
                {
                    for j in possible.combine_possible() {
                        str.push(Jamo::from(j).into());
                        str.push_str("  ");
                    }
                    Some(str)
                } else {
                    Some(str)
                }
            }
            2 => {
                let mut str: String = "".into();
                if let Some(possible) = self.syllable.read().unwrap().finale() {
                    for j in possible.append_possible() {
                        str.push_str(&format!("{:<4}", Jamo::from(j).rr()));
                    }
                    Some(str)
                } else if let Some(possible) =
                    self.syllable.read().unwrap().medial()
                {
                    for j in possible.combine_possible() {
                        str.push_str(&format!("{:<4}", Jamo::from(j).rr()));
                    }
                    Some(str)
                } else {
                    Some(str)
                }
            }
            _ => None,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct JamoInfo((u16, u16, u16));
impl JamoInfo {
    // Assuming Jamo take 'two' slots
    const LINES: [&str; 15] = [
        " Initials/Finals        Medials         ",
        " ㄱ g                   ㅏ a            ",
        " ㄴ n                   ㅐ ae           ",
        " ㄷ d                   ㅑ ya           ",
        " ㄹ r                   ㅒ yae          ",
        " ㅁ m                   ㅓ eo           ",
        " ㅂ b                   ㅔ e            ",
        " ㅅ s                   ㅕ yeo          ",
        " ㅇ ng                  ㅖ ye           ",
        " ㅈ j                   ㅗ o            ",
        " ㅊ ch                  ㅛ yo           ",
        " ㅋ k                   ㅜ u            ",
        " ㅌ t                   ㅠ yu           ",
        " ㅍ p                   ㅡ eu           ",
        " ㅎ h                   ㅣ i            ",
    ];
    pub fn new(pos: (u16, u16, u16)) -> Self {
        Self(pos)
    }
}
impl Block for JamoInfo {
    fn pos(&self) -> (u16, u16, u16) {
        self.0
    }

    fn rel_line(&self, i: u16) -> Option<String> {
        Self::LINES.get(i as usize).map(|&s| s.into())
    }
}

/*
ㄱ g
ㄲ gg
ㄳ gs
ㄴ n
ㄵ nc
ㄶ nch
ㄷ d
ㄸ dd
ㄹ r
ㄺ lg
ㄻ lm
ㄼ lb
ㄽ ls
ㄾ lt
ㄿ lph
ㅀ lh
ㅁ m
ㅂ b
ㅃ bb
ㅄ bs
ㅅ s
ㅆ ss
ㅇ ng
ㅈ j
ㅉ jj
ㅊ ch
ㅋ k
ㅌ t
ㅍ p
ㅎ h
 */

/*
ㄱ g
ㄴ n
ㄷ d
ㄹ r
ㅁ m
ㅂ b
ㅅ s
ㅇ ng
ㅈ j
ㅊ ch
ㅋ k
ㅌ t
ㅍ p
ㅎ h
 */

/*
ㄱ g
ㄴ n
ㄷ d
ㄹ r
ㅁ m
ㅂ b
ㅅ s
ㅇ ng
ㅈ j
ㅊ ch
ㅋ k
ㅌ t
ㅍ p
ㅎ h
 */

/*
ㅏ a
ㅐ ae
ㅑ ya
ㅒ yae
ㅓ eo
ㅔ e
ㅕ yeo
ㅖ ye
ㅗ o
ㅛ yo
ㅜ u
ㅠ yu
ㅡ eu
ㅣ i
 */
