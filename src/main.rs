#![allow(dead_code)]

use terminal::{Terminal, TerminalResult};

use crate::scenes::main_scene;

mod elements;
mod ext;
mod hangul;
mod hangul_parser;
mod jamo;
mod scenes;
mod syllable;

// Assuming char is 1:2
// 4:3 becomes 8:3
fn main() -> TerminalResult<()> {
    pretty_env_logger::init();
    let mut term = Terminal::new("main".into(), main_scene()?);
    term.run((81, 31))
}
