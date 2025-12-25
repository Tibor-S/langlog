use std::u16;

use terminal::{
    Scene, SceneType, TerminalResult,
    code::TerminalCode,
    elements::{Button, Rectangle, TextLine},
};

const WIDTH: u16 = 57;
const HEIGHT: u16 = 21;
const MARGIN: u16 = 3;
const MARGIN_2: u16 = 2 * MARGIN;
const HEADING: &str = ":::Menu:::";
const HEADING_LEN: u16 = 10;
const DELETE: &str = "Delete";
const DELETE_LEN: u16 = 6;
const SEARCH: &str = "Search";
const SEARCH_LEN: u16 = 6;
const CLOSE: &str = "Close";
const CLOSE_LEN: u16 = 5;

pub fn menu_scene() -> TerminalResult<Scene> {
    let mut scene = Scene::new(SceneType::PopUp(12, 5));
    scene.insert_block(
        "background".into(),
        Rectangle::new((0, 0, 0), (WIDTH, HEIGHT), true),
    )?;
    scene.insert_block(
        "heading".into(),
        TextLine::default()
            .with_pos(centered_x(HEADING_LEN), 1)
            .with_width(u16::MAX)
            .with_value(HEADING.into())
            .clone(),
    )?;
    scene.insert_input(Button::new(
        (centered_x(SEARCH_LEN + MARGIN_2), 3, 0),
        SEARCH.into(),
        SEARCH_LEN + MARGIN_2,
        MARGIN,
        Some(|| TerminalCode::None),
    ));
    scene.insert_input(Button::new(
        (centered_x(DELETE_LEN + MARGIN_2), 5, 0),
        DELETE.into(),
        DELETE_LEN + MARGIN_2,
        MARGIN,
        Some(|| TerminalCode::None),
    ));
    scene.insert_input(Button::new(
        (centered_x(CLOSE_LEN + MARGIN_2), HEIGHT - 3, 0),
        CLOSE.into(),
        CLOSE_LEN + MARGIN_2,
        MARGIN,
        Some(|| TerminalCode::PreviousScene),
    ));
    Ok(scene)
}

pub fn centered_x(width: u16) -> u16 {
    (WIDTH / 2).saturating_sub(width / 2)
}
