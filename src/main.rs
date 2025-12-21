#![allow(dead_code)]

use terminal::{
    Terminal, TerminalResult,
    elements::{LineHorizontal, LineVertical, TextLine},
};

mod ext;
mod hangul;
mod hangul_parser;
mod jamo;
mod syllable;

// Assuming char is 1:2
// 4:3 becomes 8:3
fn main() -> TerminalResult<()> {
    pretty_env_logger::init();
    let mut term = Terminal::new();
    term.insert_block(
        "mid-line-v".into(),
        LineVertical::default()
            .with_x(40)
            .with_line_start(0)
            .with_length(31)
            .clone(),
    )?;
    term.insert_block(
        "mid-line-h".into(),
        LineHorizontal::default()
            .with_y(15)
            .with_line_start(0)
            .with_length(41)
            .clone(),
    )?;
    term.insert_input(
        TextLine::default().with_pos(5, 5).with_width(24).clone(),
    );
    term.run((81, 31))
}
