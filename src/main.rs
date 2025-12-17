#![allow(dead_code)]

use terminal::{Terminal, TerminalResult};

mod ext;
mod hangul;
mod hangul_parser;
mod jamo;
mod syllable;

fn main() -> TerminalResult<()> {
    pretty_env_logger::init();
    let mut term = Terminal::new();
    term.run()
}
