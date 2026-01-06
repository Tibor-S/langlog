use std::{rc::Rc, sync::RwLock, u16};

use isahc::{Request, RequestExt, http::StatusCode};
use terminal::{
    Scene, SceneType, TerminalResult,
    code::TerminalCode,
    elements::{Button, Dispatch, Rectangle, TextLine},
};

use crate::{
    Client, HttpResult,
    elements::{HangulResult, Log, RrInput},
    hangul::Hangul,
    host::{DeleteEntry, database::Account},
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
    client: Rc<RwLock<Client>>,
    account: Rc<RwLock<Option<Account>>>,
) -> TerminalResult<(Scene, Vec<(String, Scene)>)> {
    let mut scene = Scene::new(SceneType::PopUp(12, 5));
    scene.insert_block(
        "background".into(),
        Rectangle::new((0, 0, 0), (WIDTH, HEIGHT), true, None),
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
    let delete_scene = delete_scene(client, account)?;
    let not_found_error = error_popup_scene(
        full_wh,
        "Could not find given entry!".into(),
        &[],
        true,
    )?;
    let no_account_error =
        error_popup_scene(full_wh, "Not signed in".into(), &[], true)?;
    Ok((
        scene,
        vec![
            ("find-menu".into(), find_scene),
            ("delete-menu".into(), delete_scene),
            ("not-found-error".into(), not_found_error),
            ("no-account-error".into(), no_account_error),
        ],
    ))
}

fn find_scene(log: Dispatch<Log>) -> TerminalResult<Scene> {
    let mut scene = Scene::new(SceneType::PopUp(12, 5));
    scene.insert_block(
        "background".into(),
        Rectangle::new((0, 0, 0), (WIDTH, HEIGHT), true, None),
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

fn delete_scene(
    client: Rc<RwLock<Client>>,
    account: Rc<RwLock<Option<Account>>>,
) -> TerminalResult<Scene> {
    let mut scene = Scene::new(SceneType::PopUp(12, 5));
    scene.insert_block(
        "background".into(),
        Rectangle::new((0, 0, 0), (WIDTH, HEIGHT), true, None),
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
            let mut rr = rr.write().unwrap_or_else(|e| e.into_inner());
            {
                let hangul_res = rr.hangul();
                let hangul_guard =
                    hangul_res.read().unwrap_or_else(|e| e.into_inner());
                let hangul = hangul_guard.str();

                if hangul.is_empty() {
                    return TerminalCode::None;
                }

                let client = client.read().unwrap_or_else(|e| e.into_inner());
                let client = &*client;
                let account = account.read().unwrap_or_else(|e| e.into_inner());

                let account = if let Some(account) = account.as_ref() {
                    account
                } else {
                    return TerminalCode::ReplaceCurrentScene(
                        "no-account-error".into(),
                    );
                };

                let res = delete_entry(client, account, hangul);
                if !matches!(res, Ok(true)) {
                    return TerminalCode::GoToScene("not-found-error".into());
                }
            }
            rr.clear();
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

// true if deletion was succesful
// and reload is required
fn delete_entry(
    client: &Client,
    account: &Account,
    hangul: &Hangul,
) -> HttpResult<bool> {
    Ok(Request::delete(format!("{}/log/delete", client.remote))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&DeleteEntry {
            account: account.clone(),
            hangul: hangul.to_string(),
        })?)?
        .send()?
        .status()
        == StatusCode::RESET_CONTENT)
}
