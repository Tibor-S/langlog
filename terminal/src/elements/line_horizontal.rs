use crate::traits::Block;

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
