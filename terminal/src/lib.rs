pub mod code;
pub mod elements;
pub mod ext;
pub mod traits;
pub use crossterm::event;
pub use crossterm::style;
use crossterm::style::StyledContent;

use std::ops::Range;
use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::{self},
    io::{self, Stdout, Write},
};

use crossterm::{
    cursor,
    event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    queue, terminal,
};

use crate::ext::call_unary;
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
macro_rules! tab {
    () => {
        KeyEvent {
            code: KeyCode::Tab,
            kind: KeyEventKind::Press,
            ..
        }
    };
}
macro_rules! back_tab {
    () => {
        KeyEvent {
            code: KeyCode::BackTab,
            kind: KeyEventKind::Press,
            ..
        }
    };
}

pub struct Terminal<F = ()> {
    scenes: HashMap<String, Scene>,
    current_scene: String,
    previous_scenes: Vec<String>,
    last_full_scene: Vec<usize>,
    key_listener: F,
}
impl Terminal<()> {
    pub fn new(
        scene_name: String,
        scene: Scene,
    ) -> Terminal<impl Fn(KeyEvent) -> TerminalCode> {
        let mut scenes = HashMap::default();
        scenes.insert(scene_name.clone(), scene);
        Terminal {
            scenes,
            current_scene: scene_name,
            previous_scenes: Default::default(),
            last_full_scene: Default::default(),
            key_listener: |k| TerminalCode::UnhandledKey(k),
        }
    }
}
impl<F> Terminal<F>
where
    F: Fn(KeyEvent) -> TerminalCode,
{
    pub fn with_key_listener(
        scene_name: String,
        scene: Scene,
        key_listener: F,
    ) -> Self {
        let mut scenes = HashMap::default();
        scenes.insert(scene_name.clone(), scene);
        Terminal {
            scenes,
            current_scene: scene_name,
            previous_scenes: Default::default(),
            last_full_scene: Default::default(),
            key_listener,
        }
    }

    pub fn scene(&self) -> &Scene {
        &self.scenes[&self.current_scene]
    }

    pub fn scene_mut(&mut self) -> &mut Scene {
        self.scenes
            .get_mut(&self.current_scene)
            .expect("Logic error! Scene did not exist")
    }

    pub fn run(&mut self, size: (u16, u16)) -> TerminalResult<()> {
        let w = &mut Self::stdout();
        crossterm::execute!(
            w,
            terminal::EnterAlternateScreen,
            terminal::SetSize(size.0, size.1)
        )?;

        loop {
            queue!(
                w,
                style::ResetColor,
                terminal::Clear(terminal::ClearType::All),
                cursor::MoveTo(0, 0)
            )?;
            for previous_scene in self.previous_scenes
                [self.draw_previous_range()]
            .iter()
            .filter_map(|name| self.scenes.get(name))
            {
                Self::draw(previous_scene, w)?;
            }
            Self::draw(self.scene(), w)?;
            self.focus_cursor(w)?;
            w.flush()?;
            match self.read()? {
                TerminalCode::PreviousScene => {
                    self.previous_scene();
                }
                TerminalCode::PreviousSceneWithFocus(i) => {
                    self.previous_scene();
                    self.scene_mut().focus_input(i)?;
                }
                TerminalCode::GoToScene(name) => {
                    self.go_to_scene(name);
                }
                TerminalCode::ReplaceCurrentScene(name) => {
                    self.current_scene = name
                }
                TerminalCode::Focus(i) => {
                    self.scene_mut().focus_input(i)?;
                }
                TerminalCode::FocusAt(pos) => {
                    self.scene_mut().focus_input_at(pos)?;
                }
                TerminalCode::Exit => break,
                TerminalCode::None | TerminalCode::UnhandledKey(_) => (),
            }
        }

        crossterm::execute!(
            w,
            style::ResetColor,
            cursor::Show,
            terminal::LeaveAlternateScreen
        )?;

        Ok(())
    }

    pub fn insert_scene(&mut self, name: String, scene: Scene) {
        self.scenes.insert(name, scene);
    }

    pub fn go_to_scene(&mut self, scene: String) {
        if self.current_scene == scene {
            return;
        }
        if matches!(self.scene().ty, SceneType::Full) {
            self.last_full_scene.push(self.previous_scenes.len());
        }
        self.previous_scenes.push(self.current_scene.clone());
        self.current_scene = scene;
    }

    pub fn previous_scene(&mut self) {
        if let Some(scene) = self.previous_scenes.pop() {
            self.current_scene = scene;
        }
        if Some(self.previous_scenes.len()) == self.get_last_full_scene() {
            self.last_full_scene.pop();
        }
    }

    fn draw(scene: &Scene, w: &mut Stdout) -> TerminalResult<()> {
        let (x, y) = scene.pos();
        let blocks = scene
            .blocks()
            .iter()
            .map(|b| b.as_ref())
            .chain(scene.inputs().iter().map(|i| i.as_ref() as &dyn Block));
        for block in blocks {
            let (bx, by, _) = block.pos();
            let (x, y) = (x + bx, y + by);
            queue!(w, cursor::MoveTo(x, y))?;
            let mut i = 0;
            while let Some(line) = block.rel_line(i) {
                let styles = block.style_line(i);
                let mut print_range = 0..line.len();
                i += 1;

                // Print
                for (mut range, style) in styles {
                    // Safety check
                    range.start = range.start.min(print_range.end);
                    range.end = range.end.min(print_range.end);
                    // Let print_range catch up with range
                    let until = print_range.start..range.start;
                    if !until.is_empty() {
                        queue!(w, style::Print(&line[until]),)?;
                    }

                    print_range.start = range.end;
                    queue!(
                        w,
                        style::PrintStyledContent(StyledContent::new(
                            style,
                            &line[range]
                        ))
                    )?;
                    if print_range.is_empty() {
                        break;
                    }
                }
                queue!(
                    w,
                    style::Print(&line[print_range]),
                    cursor::MoveTo(x, y + i)
                )?;
            }
        }
        Ok(())
    }

    fn focus_cursor(&self, w: &mut Stdout) -> TerminalResult<()> {
        let input = match self.scene().focused_input() {
            Some(input) => input,
            None => return Self::hide_cursor(w),
        };
        let (x, y) = self.scene().pos();
        if let Some((rx, ry)) = input.rel_cursor_pos() {
            let (cx, cy) = (input.input_pos());
            queue!(w, cursor::Show, cursor::MoveTo(x + rx + cx, y + ry + cy))
                .map_err(TerminalError::from)
        } else {
            Self::hide_cursor(w)
        }
    }

    fn read(&mut self) -> TerminalResult<TerminalCode> {
        // Terminal match
        let code = match Self::read_key()? {
            ctrl!('q') => TerminalCode::Exit,
            back_tab!() if !self.scene().inputs.is_empty() => self
                .scene_mut()
                .focus_prev_input()
                .map(|_| TerminalCode::None)?,
            tab!() if !self.scene().inputs.is_empty() => self
                .scene_mut()
                .focus_next_input()
                .map(|_| TerminalCode::None)?,
            key => TerminalCode::UnhandledKey(key),
        };
        let key = match code {
            TerminalCode::UnhandledKey(k) => k,
            c => return Ok(c),
        };

        // Program match
        let code = call_unary(&self.key_listener, key);
        let key = match code {
            TerminalCode::UnhandledKey(k) => k,
            c => return Ok(c),
        };

        // Input match
        Ok(self.feed_focused(key))
    }

    fn feed_focused(&mut self, key: KeyEvent) -> TerminalCode {
        match self.scene().focused {
            None => TerminalCode::UnhandledKey(key),
            Some(i) => self.scene_mut().inputs[i].feed(key),
        }
    }

    fn hide_cursor(w: &mut Stdout) -> TerminalResult<()> {
        queue!(w, cursor::Hide).map_err(TerminalError::from)
    }

    fn read_key() -> TerminalResult<KeyEvent> {
        loop {
            if let Ok(Event::Key(event)) = event::read() {
                return Ok(event);
            }
        }
    }
    fn get_last_full_scene(&self) -> Option<usize> {
        self.last_full_scene.last().cloned()
    }
    fn draw_previous_range(&self) -> Range<usize> {
        match self.last_full_scene.last() {
            Some(&index) => index..self.previous_scenes.len(),
            None => 0..0,
        }
    }
    fn stdout() -> Stdout {
        io::stdout()
    }
}
impl<F> fmt::Debug for Terminal<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Terminal")
            .field("current_scene", &self.current_scene)
            .finish()
    }
}

#[derive(Debug, Clone, Default)]
pub enum SceneType {
    #[default]
    Full,
    PopUp(u16, u16), // Position for pop-up, i.e, all blocks will be drawn relative to given position
}
#[derive(Default)]
pub struct Scene {
    pub(crate) blocks: Vec<Box<dyn Block>>,
    pub(crate) inputs: Vec<Box<dyn Input>>,
    pub(crate) block_names: HashMap<String, usize>,
    pub(crate) focused: Option<usize>,
    pub(crate) ty: SceneType,
}
impl Scene {
    pub fn new(ty: SceneType) -> Self {
        Self {
            ty,
            ..Default::default()
        }
    }

    pub fn blocks(&self) -> &[Box<dyn Block>] {
        &self.blocks
    }

    pub fn inputs(&self) -> &[Box<dyn Input + 'static>] {
        &self.inputs
    }

    pub fn block_names(&self) -> &HashMap<String, usize> {
        &self.block_names
    }

    pub fn focused(&self) -> Option<usize> {
        self.focused
    }

    pub fn pos(&self) -> (u16, u16) {
        match self.ty {
            SceneType::Full => (0, 0),
            SceneType::PopUp(x, y) => (x, y),
        }
    }
}
/// Block functions
impl Scene {
    pub fn insert_block<B: Block + 'static>(
        &mut self,
        name: String,
        block: B,
    ) -> TerminalResult<()> {
        if self.block_names.contains_key(&name) {
            return Err(TerminalError::NameExists(name));
        }
        let boxed: Box<dyn Block> = Box::new(block);
        let (_, upper) = self.blocks_pos_search(&boxed.pos());
        self.blocks.insert(upper, boxed);
        for (_, index) in self.block_names.iter_mut() {
            if *index >= upper {
                *index += 1;
            }
        }
        self.block_names.insert(name, upper);
        Ok(())
    }

    pub fn remove_block(&mut self, name: &String) -> Option<Box<dyn Block>> {
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

    pub fn get_block(&mut self, name: &String) -> Option<&Box<dyn Block>> {
        let index = match self.block_names.get(name) {
            Some(i) => *i,
            None => return None,
        };
        self.blocks.get(index)
    }

    /// Returns `(A, B)`
    /// `A` is `Some(index)` if `pos` is found
    /// `B` is the upper bound for `pos`
    fn blocks_pos_search(
        &self,
        pos: &(u16, u16, u16),
    ) -> (Option<usize>, usize) {
        let ub = upper_bound(&self.blocks, pos, |(vx, vy, vz), block| {
            let (bx, by, bz) = &block.pos();
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
        if self.blocks[index].pos() == *pos {
            (Some(index), ub)
        } else {
            (None, ub)
        }
    }
}
/// Input functions
impl Scene {
    pub fn insert_input<I: Input + 'static>(
        &mut self,
        input: I,
    ) -> Option<Box<dyn Input>> {
        let pos = input.input_pos();
        let boxed: Box<dyn Input> = Box::new(input);
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

    pub fn remove_input(&mut self, pos: (u16, u16)) -> Option<Box<dyn Input>> {
        match self.get_input_at_pos(pos) {
            Ok((i, _)) => Some(self.inputs.remove(i)),
            Err(_) => None,
        }
    }

    pub fn focused_input(&self) -> Option<&dyn Input> {
        self.focused.map(|i| self.inputs[i].as_ref())
    }

    pub fn focused_input_mut(&mut self) -> Option<&mut dyn Input> {
        let index = match self.focused {
            Some(v) => v,
            None => return None,
        };
        Some(self.inputs[index].as_mut())
    }

    pub fn focus_prev_input(&mut self) -> TerminalResult<&dyn Input> {
        match self.focused {
            None => self.focus_input(self.inputs.len() - 1),
            Some(0) => self.focus_input(0),
            Some(i) => self.focus_input(i - 1),
        }
    }

    pub fn focus_next_input(&mut self) -> TerminalResult<&dyn Input> {
        match self.focused {
            None => self.focus_input(0),
            Some(i) if i == self.inputs.len() - 1 => self.focus_input(i),
            Some(i) => self.focus_input(i + 1),
        }
    }

    pub fn focus_input_at(
        &mut self,
        pos: (u16, u16),
    ) -> TerminalResult<&dyn Input> {
        match self.get_input_at_pos(pos) {
            Ok((i, _)) => self.focus_input(i),
            Err(_) => Err(TerminalError::NoInputAt(pos)),
        }
    }

    fn get_input_at_pos(
        &mut self,
        pos: (u16, u16),
    ) -> Result<(usize, &dyn Input), usize> {
        let index = match self.inputs.binary_search_by(|inp| {
            let (x, y) = inp.input_pos();
            match y.cmp(&pos.1) {
                std::cmp::Ordering::Equal => x.cmp(&pos.0),
                ord => ord,
            }
        }) {
            Ok(i) => i,
            Err(i) => return Err(i),
        };

        Ok((index, self.inputs[index].as_ref()))
    }

    fn focus_input(&mut self, i: usize) -> TerminalResult<&dyn Input> {
        // Safety check
        if i >= self.inputs().len() {
            return Err(TerminalError::NoInput(i));
        }

        // Signal unfocus previous input
        if let Some(prev_index) = self.focused {
            self.inputs[prev_index].unfocus();
        }

        self.focused = Some(i);
        let input = self.inputs[i].as_mut();
        input.focus();
        Ok(input)
    }

    /*


    */
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
