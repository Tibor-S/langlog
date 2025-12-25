use std::u16;

use terminal::{
    Scene, SceneType, TerminalResult,
    code::TerminalCode,
    elements::{Button, Dispatch, Rectangle, TextLine},
};

use crate::{
    elements::{HangulResult, Log, RrInput},
    scenes::error_popup_scene,
};

const WIDTH: u16 = 57;
const HEIGHT: u16 = 21;
const MARGIN: u16 = 3;
const MARGIN_2: u16 = 2 * MARGIN;
const HEADING: &str = ":::Menu:::";
const HEADING_LEN: u16 = 10;
const DELETE: &str = "Delete";
const DELETE_LEN: u16 = 6;
const FIND: &str = "Find";
const FIND_LEN: u16 = 4;
const CLOSE: &str = "Close";
const CLOSE_LEN: u16 = 5;

pub fn menu_scene(
    full_wh: (u16, u16),
    log: Dispatch<Log>,
) -> TerminalResult<(Scene, Vec<(String, Scene)>)> {
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
        (centered_x(FIND_LEN + MARGIN_2), 3, 0),
        FIND.into(),
        FIND_LEN + MARGIN_2,
        MARGIN,
        Some(|| TerminalCode::ReplaceCurrentScene("find-menu".into())),
    ));
    scene.insert_input(Button::new(
        (centered_x(DELETE_LEN + MARGIN_2), 5, 0),
        DELETE.into(),
        DELETE_LEN + MARGIN_2,
        MARGIN,
        Some(|| TerminalCode::ReplaceCurrentScene("delete-menu".into())),
    ));
    scene.insert_input(Button::new(
        (centered_x(CLOSE_LEN + MARGIN_2), HEIGHT - 3, 0),
        CLOSE.into(),
        CLOSE_LEN + MARGIN_2,
        MARGIN,
        Some(|| TerminalCode::PreviousScene),
    ));
    let find_scene = find_scene(log.clone())?;
    let delete_scene = delete_scene(log)?;
    let not_found_error = error_popup_scene(
        full_wh,
        "Could not find given entry!".into(),
        &[],
        true,
    )?;
    Ok((
        scene,
        vec![
            ("find-menu".into(), find_scene),
            ("delete-menu".into(), delete_scene),
            ("not-found-error".into(), not_found_error),
        ],
    ))
}

fn find_scene(log: Dispatch<Log>) -> TerminalResult<Scene> {
    let mut scene = Scene::new(SceneType::PopUp(12, 5));
    scene.insert_block(
        "background".into(),
        Rectangle::new((0, 0, 0), (WIDTH, HEIGHT), true),
    )?;
    scene.insert_block(
        "heading".into(),
        TextLine::default()
            .with_pos(centered_x(FIND_LEN), 1)
            .with_width(u16::MAX)
            .with_value(FIND.into())
            .clone(),
    )?;
    /*
     * Hangul
     */
    let hangul_result = {
        let h = Dispatch::from(HangulResult::new((centered_x(10), 3, 0)));
        scene.insert_block("hangul".into(), h.clone())?;
        h
    };
    /*
     * rr
     */
    let rr = {
        let rr = Dispatch::from(RrInput::new(
            TextLine::default()
                .with_pos(centered_x(10), 4)
                .with_width(10)
                .clone(),
            hangul_result.clone(),
        ));
        scene.insert_input(rr.clone());
        rr
    };
    /*
     * Button
     */
    scene.insert_input(Button::new(
        (centered_x(FIND_LEN + MARGIN_2), 6, 0),
        FIND.into(),
        FIND_LEN + MARGIN_2,
        MARGIN,
        Some(move || {
            let found = log
                .write()
                .unwrap()
                .index_at(rr.write().unwrap().hangul().read().unwrap().str());
            rr.write().unwrap().clear();
            if found {
                TerminalCode::PreviousSceneWithFocus(3)
            } else {
                TerminalCode::ReplaceCurrentScene("not-found-error".into())
            }
        }),
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

fn delete_scene(log: Dispatch<Log>) -> TerminalResult<Scene> {
    let mut scene = Scene::new(SceneType::PopUp(12, 5));
    scene.insert_block(
        "background".into(),
        Rectangle::new((0, 0, 0), (WIDTH, HEIGHT), true),
    )?;
    scene.insert_block(
        "heading".into(),
        TextLine::default()
            .with_pos(centered_x(DELETE_LEN), 1)
            .with_width(u16::MAX)
            .with_value(DELETE.into())
            .clone(),
    )?;
    /*
     * Hangul
     */
    let hangul_result = {
        let h = Dispatch::from(HangulResult::new((centered_x(10), 3, 0)));
        scene.insert_block("hangul".into(), h.clone())?;
        h
    };
    /*
     * rr
     */
    let rr = {
        let rr = Dispatch::from(RrInput::new(
            TextLine::default()
                .with_pos(centered_x(10), 4)
                .with_width(10)
                .clone(),
            hangul_result.clone(),
        ));
        scene.insert_input(rr.clone());
        rr
    };
    /*
     * Button
     */
    scene.insert_input(Button::new(
        (centered_x(DELETE_LEN + MARGIN_2), 6, 0),
        DELETE.into(),
        DELETE_LEN + MARGIN_2,
        MARGIN,
        Some(move || {
            log.write().unwrap().remove_entry(
                rr.write().unwrap().hangul().read().unwrap().str(),
            );
            rr.write().unwrap().clear();
            TerminalCode::PreviousScene
        }),
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

fn centered_x(width: u16) -> u16 {
    (WIDTH / 2).saturating_sub(width / 2)
}
