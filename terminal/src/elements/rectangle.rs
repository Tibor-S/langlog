use std::iter::repeat;

use crate::traits::Block;

#[derive(Debug, Clone, Default)]
pub struct Rectangle {
    pos: (u16, u16, u16),
    dim_wh: (u16, u16),
    bordered: bool,
}
impl Rectangle {
    pub fn new(
        pos: (u16, u16, u16),
        dim_wh: (u16, u16),
        bordered: bool,
    ) -> Self {
        Self {
            pos,
            dim_wh,
            bordered,
        }
    }
}
impl Block for Rectangle {
    fn pos(&self) -> (u16, u16, u16) {
        self.pos
    }

    fn rel_line(&self, i: u16) -> Option<String> {
        if self.dim_wh.1 == 0 {
            return None;
        }
        if !self.bordered && i < self.dim_wh.1 {
            return Some(repeat(' ').take(self.dim_wh.0 as usize).collect());
        }
        if i == 0 || i == self.dim_wh.1 - 1 {
            Some(
                Some('+')
                    .into_iter()
                    .chain(
                        repeat('―')
                            .take(self.dim_wh.0.saturating_sub(2) as usize),
                    )
                    .chain(Some('+'))
                    .collect(),
            )
        } else if i < self.dim_wh.1 {
            Some(
                Some('│')
                    .into_iter()
                    .chain(
                        repeat(' ')
                            .take(self.dim_wh.0.saturating_sub(2) as usize),
                    )
                    .chain(Some('│'))
                    .collect(),
            )
        } else {
            None
        }
    }
}
