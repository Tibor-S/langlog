use std::iter::repeat;

use crate::{elements::TextLine, traits::Block};

#[derive(Debug, Clone, Default)]
pub struct Rectangle {
    pos: (u16, u16, u16),
    dim_wh: (u16, u16),
    bordered: bool,
    heading: Option<TextLine>,
}
impl Rectangle {
    pub fn new(
        pos: (u16, u16, u16),
        dim_wh: (u16, u16),
        bordered: bool,
        heading: Option<TextLine>,
    ) -> Self {
        let mut heading = heading;
        if let Some(heading) = heading.as_mut() {
            heading.display_width =
                heading.display_width.min(dim_wh.0.saturating_sub(4))
        }
        Self {
            pos,
            dim_wh,
            bordered,
            heading,
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
            Some(repeat(' ').take(self.dim_wh.0 as usize).collect())
        } else if i == 0
            && let Some(heading) = self.heading.as_ref()
        {
            let text_line_rel_line = heading.rel_line(0);
            let text_iter = text_line_rel_line.as_ref().map(|t| {
                Some(' ').into_iter().chain(t.chars()).chain(Some(' '))
            });

            if let Some(iter) = text_iter {
                Some(
                    Some('+')
                        .into_iter()
                        .chain(
                            Some('―')
                                .into_iter()
                                .chain(iter)
                                .chain(repeat('―')),
                        )
                        .take(self.dim_wh.0.saturating_sub(1) as usize)
                        .chain(Some('+'))
                        .collect(),
                )
            } else {
                Some(
                    Some('+')
                        .into_iter()
                        .chain(repeat('―'))
                        .take(self.dim_wh.0.saturating_sub(1) as usize)
                        .chain(Some('+'))
                        .collect(),
                )
            }
        } else if i == 0 || i == self.dim_wh.1 - 1 {
            Some(
                Some('+')
                    .into_iter()
                    .chain(repeat('―'))
                    .take(self.dim_wh.0.saturating_sub(1) as usize)
                    .chain(Some('+'))
                    .collect(),
            )
        } else if i < self.dim_wh.1 {
            Some(
                Some('│')
                    .into_iter()
                    .chain(repeat(' '))
                    .take(self.dim_wh.0.saturating_sub(1) as usize)
                    .chain(Some('│'))
                    .collect(),
            )
        } else {
            None
        }
    }
}
