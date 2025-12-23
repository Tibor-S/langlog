use std::ops::Range;

use terminal::{
    style::{Attribute, Attributes, ContentStyle},
    traits::Block,
};

use crate::{hangul::Hangul, hangul_parser::HangulParser, syllable::Syllable};

#[derive(Debug, Default)]
pub struct HangulResult {
    pos: (u16, u16, u16),
    overflow: String,
    syllable: Syllable,
    str: Hangul,
    parser: HangulParser,
}
impl HangulResult {
    pub fn new(pos: (u16, u16, u16)) -> Self {
        Self {
            pos,
            ..Default::default()
        }
    }

    pub fn syllable(&self) -> &Syllable {
        &self.syllable
    }

    pub fn str(&self) -> &Hangul {
        &self.str
    }

    pub fn overflow(&self) -> &String {
        &self.overflow
    }

    pub fn is_empty(&self) -> bool {
        self.str.is_empty()
    }

    pub fn push(&mut self) {
        self.str.push(self.syllable.clone());
    }

    pub fn pop(&mut self) {}

    pub fn set_rr(&mut self, rr: &str) {
        let overflow;
        (self.syllable, overflow) = self.parser.parse_syllable(rr);
        self.overflow = overflow.into();
    }

    pub fn clear(&mut self) {
        self.overflow = String::new();
        self.syllable = Syllable::default();
        self.str = Hangul::default();
    }
}
impl Block for HangulResult {
    fn pos(&self) -> (u16, u16, u16) {
        self.pos
    }

    fn rel_line(&self, i: u16) -> Option<String> {
        match i {
            0 => Some(format!("{}{}", self.str, self.syllable)),
            _ => None,
        }
    }

    fn style_line(&self, i: u16) -> Vec<(Range<usize>, ContentStyle)> {
        if i != 0 {
            return vec![];
        }
        // utf-8 byte count
        let str = format!("{}", self.str);
        let ex = format!("{}", self.syllable);
        let style = ContentStyle {
            attributes: Attributes::none().with(Attribute::RapidBlink),
            ..Default::default()
        };
        vec![(str.len()..(str.len() + ex.len()), style)]
    }
}
