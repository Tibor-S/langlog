#![allow(dead_code)]

use terminal::{
    Terminal, TerminalResult,
    elements::{LineHorizontal, LineVertical, TextLine},
};

use crate::elements::{HangulInput, JamoInfo, PossibleInfo};

mod elements;
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
        "mid-line-h1".into(),
        LineHorizontal::default()
            .with_y(11)
            .with_line_start(0)
            .with_length(41)
            .clone(),
    )?;
    term.insert_block(
        "mid-line-h2".into(),
        LineHorizontal::default()
            .with_y(15)
            .with_line_start(0)
            .with_length(41)
            .clone(),
    )?;
    term.insert_block(
        "top-line-1".into(),
        LineHorizontal::default()
            .with_y(0)
            .with_line_start(0)
            .with_length(41)
            .clone(),
    )?;
    term.insert_block(
        "top-line-2".into(),
        LineHorizontal::default()
            .with_y(0)
            .with_line_start(40)
            .with_length(41)
            .clone(),
    )?;
    term.insert_block(
        "command-sep-bot".into(),
        LineHorizontal::default()
            .with_y(2)
            .with_line_start(0)
            .with_length(41)
            .clone(),
    )?;
    let hangul_input = HangulInput::new((5, 5, 0), 24);
    term.insert_block(
        "combinations".into(),
        PossibleInfo::new((1, 12, 0), hangul_input.syllable()),
    )?;
    term.insert_input(hangul_input);
    term.insert_block(
        "info-line".into(),
        TextLine::default()
            .with_pos(1, 1)
            .with_width("Exit: ^q    Command Menu: ^Space".len() as u16)
            .with_value("Exit: ^q    Command Menu: ^Space".into())
            .clone(),
    )?;
    term.insert_block("jamo-box".into(), JamoInfo::new((0, 16, 0)))?;
    term.run((81, 31))
}
