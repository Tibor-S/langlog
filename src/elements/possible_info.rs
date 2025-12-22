use terminal::{elements::Dispatch, traits::Block};

use crate::{elements::HangulResult, jamo::Jamo};

pub struct PossibleInfo {
    pos: (u16, u16, u16),
    hangul_result: Dispatch<HangulResult>,
}
impl PossibleInfo {
    pub fn new(
        pos: (u16, u16, u16),
        hangul_result: Dispatch<HangulResult>,
    ) -> Self {
        Self { pos, hangul_result }
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
                if let Some(possible) =
                    self.hangul_result.read().unwrap().syllable().finale()
                {
                    for j in possible.append_possible() {
                        str.push(Jamo::from(j).into());
                        str.push_str("  ");
                    }
                    Some(str)
                } else if let Some(possible) =
                    self.hangul_result.read().unwrap().syllable().medial()
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
                if let Some(possible) =
                    self.hangul_result.read().unwrap().syllable().finale()
                {
                    for j in possible.append_possible() {
                        str.push_str(&format!("{:<4}", Jamo::from(j).rr()));
                    }
                    Some(str)
                } else if let Some(possible) =
                    self.hangul_result.read().unwrap().syllable().medial()
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
