pub mod code;
pub mod elements;
mod ext;
pub mod traits;

use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::{self},
    io::{self, Stdout, Write},
    rc::Rc,
    sync::RwLock,
};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    queue, style, terminal,
};

use crate::{
    code::TerminalCode,
    ext::upper_bound,
    traits::{Block, Input},
};

macro_rules! ctrl {
    ($c:expr) => {
        KeyEvent {
            code: KeyCode::Char($c),
            modifiers: KeyModifiers::CONTROL,
            ..
        }
    };
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

pub struct Terminal {
    stdout: Stdout,
    blocks: Vec<Rc<RwLock<dyn Block>>>,
    inputs: Vec<Rc<RwLock<dyn Input>>>,
    block_names: HashMap<String, usize>,
    focused: Option<usize>,
}
impl Terminal {
    pub fn new() -> Self {
        Self {
            stdout: io::stdout(),
            blocks: vec![],
            block_names: HashMap::new(),
            inputs: vec![],
            focused: None,
        }
    }

    pub fn run(&mut self, size: (u16, u16)) -> TerminalResult<()> {
        crossterm::execute!(
            self.stdout,
            terminal::EnterAlternateScreen,
            terminal::SetSize(size.0, size.1)
        )?;

        loop {
            queue!(
                self.stdout,
                style::ResetColor,
                terminal::Clear(terminal::ClearType::All),
                cursor::MoveTo(0, 0)
            )?;
            self.draw()?;
            self.focus_cursor()?;
            self.stdout.flush()?;
            match self.read()? {
                TerminalCode::Exit => break,
                _ => (),
            }
        }

        crossterm::execute!(
            self.stdout,
            style::ResetColor,
            cursor::Show,
            terminal::LeaveAlternateScreen
        )?;

        Ok(())
    }

    fn draw(&mut self) -> TerminalResult<()> {
        for block in self.blocks.iter() {
            let (x, y, _) = block.read().unwrap().pos();
            queue!(self.stdout, cursor::MoveTo(x, y))?;
            let mut i = 0;
            while let Some(line) = block.read().unwrap().rel_line(i) {
                i += 1;
                queue!(
                    self.stdout,
                    style::Print(line),
                    cursor::MoveTo(x, y + i)
                )?;
            }
        }
        for block in self.inputs.iter() {
            let block = block.read().unwrap();
            let (x, y, _) = block.pos();
            queue!(self.stdout, cursor::MoveTo(x, y))?;
            let mut i = 0;
            while let Some(line) = block.rel_line(i) {
                i += 1;
                queue!(
                    self.stdout,
                    style::Print(line),
                    cursor::MoveTo(x, y + i)
                )?;
            }
        }
        Ok(())
    }

    fn focus_cursor(&mut self) -> TerminalResult<()> {
        let input = match self.focused_input() {
            Some(input) => Rc::clone(input),
            None => return self.hide_cursor(),
        };
        if let Some((rx, ry)) = input.read().unwrap().rel_cursor_pos() {
            let (x, y) = input.read().unwrap().input_pos();
            queue!(self.stdout, cursor::Show, cursor::MoveTo(x + rx, y + ry))
                .map_err(TerminalError::from)
        } else {
            self.hide_cursor()
        }
    }

    fn hide_cursor(&mut self) -> TerminalResult<()> {
        queue!(self.stdout, cursor::Hide).map_err(TerminalError::from)
    }

    fn read(&mut self) -> TerminalResult<TerminalCode> {
        match Self::read_key()? {
            ctrl!('q') => Ok(TerminalCode::Exit),
            up!() if !self.inputs.is_empty() => {
                self.focus_prev_input().map(|_| TerminalCode::None)
            }
            down!() if !self.inputs.is_empty() => {
                self.focus_next_input().map(|_| TerminalCode::None)
            }
            key => Ok(self.feed_focused(key)),
        }
    }

    fn read_key() -> TerminalResult<KeyEvent> {
        loop {
            if let Ok(Event::Key(event)) = event::read() {
                return Ok(event);
            }
        }
    }

    fn focus_input(
        &mut self,
        i: usize,
    ) -> TerminalResult<&Rc<RwLock<dyn Input>>> {
        self.inputs
            .get(i)
            .map(|input| {
                self.focused = Some(i);
                input
            })
            .ok_or(TerminalError::NoInput(i))
    }

    fn feed_focused(&mut self, key: KeyEvent) -> TerminalCode {
        match self.focused {
            None => TerminalCode::UnhandledKey(key),
            Some(i) => self.inputs[i].write().unwrap().feed(key),
        }
    }

    fn get_input_at_pos(
        &mut self,
        pos: (u16, u16),
    ) -> Result<(usize, &Rc<RwLock<dyn Input>>), usize> {
        self.inputs
            .binary_search_by(|inp| {
                let (x, y) = inp.read().unwrap().input_pos();
                match y.cmp(&pos.1) {
                    std::cmp::Ordering::Equal => x.cmp(&pos.1),
                    ord => ord,
                }
            })
            .map(|index| (index, &self.inputs[index]))
    }

    /// Returns `(A, B)`
    /// `A` is `Some(index)` if `pos` is found
    /// `B` is the upper bound for `pos`
    fn blocks_pos_search(
        &self,
        pos: &(u16, u16, u16),
    ) -> (Option<usize>, usize) {
        let ub = upper_bound(&self.blocks, pos, |(vx, vy, vz), block| {
            let (bx, by, bz) = &block.read().unwrap().pos();
            match vz.cmp(bz) {
                Ordering::Equal => (),
                o => return o,
            }
            match vy.cmp(by) {
                Ordering::Equal => (),
                o => return o,
            }
            vx.cmp(bx)
        });
        if ub == 0 {
            return (None, ub);
        }
        let index = ub - 1;
        if self.blocks[index].read().unwrap().pos() == *pos {
            (Some(index), ub)
        } else {
            (None, ub)
        }
    }
}
impl fmt::Debug for Terminal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Terminal")
            .field("stdout", &self.stdout)
            .finish()
    }
}
/// Block functions
impl Terminal {
    pub fn insert_block<B: Block + 'static>(
        &mut self,
        name: String,
        block: B,
    ) -> TerminalResult<()> {
        if self.block_names.contains_key(&name) {
            return Err(TerminalError::NameExists(name));
        }
        let boxed: Rc<RwLock<dyn Block>> = Rc::new(RwLock::new(block));
        let (_, upper) = self.blocks_pos_search(&boxed.read().unwrap().pos());
        self.blocks.insert(upper, boxed);
        for (_, index) in self.block_names.iter_mut() {
            if *index >= upper {
                *index += 1;
            }
        }
        self.block_names.insert(name, upper);
        Ok(())
    }

    pub fn remove_block(
        &mut self,
        name: &String,
    ) -> Option<Rc<RwLock<dyn Block>>> {
        let index = match self.block_names.get(name) {
            Some(i) => *i,
            None => return None,
        };
        let block = self.blocks.remove(index);
        self.block_names.remove(name);
        for (_, index2) in self.block_names.iter_mut() {
            if *index2 >= index {
                *index2 -= 1;
            }
        }
        Some(block)
    }

    pub fn get_block(
        &mut self,
        name: &String,
    ) -> Option<&Rc<RwLock<dyn Block>>> {
        let index = match self.block_names.get(name) {
            Some(i) => *i,
            None => return None,
        };
        self.blocks.get(index)
    }
}
/// Input functions
impl Terminal {
    pub fn insert_input<I: Input + 'static>(
        &mut self,
        input: I,
    ) -> Option<Rc<RwLock<dyn Input>>> {
        let pos = input.input_pos();
        let boxed: Rc<RwLock<dyn Input>> = Rc::new(RwLock::new(input));
        let ret = match self.get_input_at_pos(pos) {
            Ok((i, _)) => {
                let el = self.inputs.remove(i);
                self.inputs.insert(i, boxed);
                Some(el)
            }
            Err(i) => {
                self.inputs.insert(i, boxed);
                None
            }
        };
        if self.inputs.len() == 1 {
            self.focus_input(0)
                .expect("Qualified: Guaranteed to be at least one input");
        }
        ret
    }

    pub fn remove_input(
        &mut self,
        pos: (u16, u16),
    ) -> Option<Rc<RwLock<dyn Input>>> {
        match self.get_input_at_pos(pos) {
            Ok((i, _)) => Some(self.inputs.remove(i)),
            Err(_) => None,
        }
    }

    pub fn focused_input(&self) -> Option<&Rc<RwLock<dyn Input>>> {
        self.focused.map(|i| &self.inputs[i])
    }

    pub fn focus_prev_input(
        &mut self,
    ) -> TerminalResult<&Rc<RwLock<dyn Input>>> {
        match self.focused {
            None => self.focus_input(self.inputs.len() - 1),
            Some(0) => self.focus_input(0),
            Some(i) => self.focus_input(i - 1),
        }
    }

    pub fn focus_next_input(
        &mut self,
    ) -> TerminalResult<&Rc<RwLock<dyn Input>>> {
        match self.focused {
            None => self.focus_input(0),
            Some(i) if i == self.inputs.len() - 1 => self.focus_input(i),
            Some(i) => self.focus_input(i + 1),
        }
    }

    pub fn focus_input_at(
        &mut self,
        pos: (u16, u16),
    ) -> TerminalResult<&Rc<RwLock<dyn Input>>> {
        match self.get_input_at_pos(pos) {
            Ok((i, _)) => self.focus_input(i),
            Err(_) => Err(TerminalError::NoInputAt(pos)),
        }
    }
}
#[derive(Debug, thiserror::Error)]
pub enum TerminalError {
    #[error("No input on index {0}")]
    NoInput(usize),
    #[error("No input at position {0:?}")]
    NoInputAt((u16, u16)),
    #[error("Name already exists: {0}")]
    NameExists(String),
    #[error(transparent)]
    IO(#[from] io::Error),
}
pub type TerminalResult<T> = Result<T, TerminalError>;
