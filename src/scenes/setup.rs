use std::u16;

use terminal::{
    Scene, SceneType, TerminalResult,
    code::TerminalCode,
    elements::{Button, Dispatch, LineHorizontal, Rectangle, TextLine},
};

use crate::{
    elements::{HangulResult, Log, RrInput},
    scenes::error_popup_scene,
};

const WIDTH: u16 = 81;
const HEIGHT: u16 = 31;
const REC_Y: u16 = 2;
const REC_WIDTH: u16 = 61;
const REC_HEIGHT: u16 = 28;

const ENTRY_REC_WIDTH: u16 = REC_WIDTH - 10;

const REMOTE_REC_Y: u16 = REC_Y + 1;
const REMOTE_HEADING_TEXT: &str = "Remote:";
const REMOTE_HEADING_WIDTH: u16 = REMOTE_HEADING_TEXT.len() as u16;
const REMOTE_POS: (u16, u16, u16) =
    (centered_x(ENTRY_REC_WIDTH) + 1, REMOTE_REC_Y + 1, 0);
const REMOTE_WIDTH: u16 = ENTRY_REC_WIDTH - 2;

const USERNAME_REC_Y: u16 = REMOTE_REC_Y + 3;
const USERNAME_HEADING_TEXT: &str = "Username:";
const USERNAME_HEADING_WIDTH: u16 = USERNAME_HEADING_TEXT.len() as u16;
const USERNAME_POS: (u16, u16, u16) =
    (centered_x(ENTRY_REC_WIDTH) + 1, USERNAME_REC_Y + 1, 0);
const USERNAME_WIDTH: u16 = ENTRY_REC_WIDTH - 2;

const PASSWORD_REC_Y: u16 = USERNAME_REC_Y + 3;
const PASSWORD_HEADING_TEXT: &str = "Password:";
const PASSWORD_HEADING_WIDTH: u16 = PASSWORD_HEADING_TEXT.len() as u16;
const PASSWORD_POS: (u16, u16, u16) =
    (centered_x(ENTRY_REC_WIDTH) + 1, PASSWORD_REC_Y + 1, 0);
const PASSWORD_WIDTH: u16 = ENTRY_REC_WIDTH - 2;

const LOGIN_Y: u16 = PASSWORD_REC_Y + 4;
const LOGIN_TEXT: &str = "Login / Signup";
const LOGIN_WIDTH: u16 = LOGIN_TEXT.len() as u16;

const DIVIDER_Y: u16 = LOGIN_Y + 1;

const POSTGRES_ROW_X: u16 = centered_x(ENTRY_REC_WIDTH);
const POSTGRES_ENTRY_WIDTH: u16 = ENTRY_REC_WIDTH / 2;
const POSTGRES_ENTRY_LEFT_X: u16 = POSTGRES_ROW_X;
const POSTGRES_ENTRY_RIGHT_X: u16 =
    POSTGRES_ROW_X + ENTRY_REC_WIDTH - POSTGRES_ENTRY_WIDTH;

const POSTGRES_ROW_1_Y: u16 = DIVIDER_Y + 1;
const POSTGRES_USER_TEXT: &str = "Postgres user:";
const POSTGRES_USER_WIDTH: u16 = POSTGRES_USER_TEXT.len() as u16;
const PG_USER_POS: (u16, u16, u16) =
    (POSTGRES_ENTRY_LEFT_X + 1, POSTGRES_ROW_1_Y + 1, 0);
const PG_USER_WIDTH: u16 = POSTGRES_ENTRY_WIDTH - 2;

const POSTGRES_PASSWORD_TEXT: &str = "Postgres password:";
const POSTGRES_PASSWORD_WIDTH: u16 = POSTGRES_PASSWORD_TEXT.len() as u16;
const PG_PASSWORD_POS: (u16, u16, u16) =
    (POSTGRES_ENTRY_RIGHT_X + 1, POSTGRES_ROW_1_Y + 1, 0);
const PG_PASSWORD_WIDTH: u16 = POSTGRES_ENTRY_WIDTH - 2;

const POSTGRES_ROW_2_Y: u16 = POSTGRES_ROW_1_Y + 3;
const POSTGRES_ADDRESS_TEXT: &str = "Postgres address:";
const POSTGRES_ADDRESS_WIDTH: u16 = POSTGRES_ADDRESS_TEXT.len() as u16;
const PG_ADDRESS_POS: (u16, u16, u16) =
    (POSTGRES_ENTRY_LEFT_X + 1, POSTGRES_ROW_2_Y + 1, 0);
const PG_ADDRESS_WIDTH: u16 = POSTGRES_ENTRY_WIDTH - 2;

const SERVE_ON_ADDRESS_TEXT: &str = "Serve on address:";
const SERVE_ON_ADDRESS_WIDTH: u16 = SERVE_ON_ADDRESS_TEXT.len() as u16;
const SERVE_ON_POS: (u16, u16, u16) =
    (POSTGRES_ENTRY_RIGHT_X + 1, POSTGRES_ROW_2_Y + 1, 0);
const SERVE_ON_WIDTH: u16 = POSTGRES_ENTRY_WIDTH - 2;

const HOST_Y: u16 = POSTGRES_ROW_2_Y + 4;
const HOST_TEXT: &str = "Host";
const HOST_WIDTH: u16 = HOST_TEXT.len() as u16;

#[allow(dead_code)]
pub struct SetupItems {
    pub remote: Dispatch<TextLine>,
    pub username: Dispatch<TextLine>,
    pub password: Dispatch<TextLine>,
    pub pg_user: Dispatch<TextLine>,
    pub pg_password: Dispatch<TextLine>,
    pub pg_address: Dispatch<TextLine>,
    pub serve_on: Dispatch<TextLine>,
}

pub fn setup_scene() -> TerminalResult<(Scene, Vec<(String, Scene)>, SetupItems)>
{
    let mut scene = Scene::new(SceneType::Full);

    let items = SetupItems {
        remote: Dispatch::from(TextLine {
            pos: REMOTE_POS,
            display_width: REMOTE_WIDTH,
            index: 0,
            value: String::new(),
        }),
        username: Dispatch::from(TextLine {
            pos: USERNAME_POS,
            display_width: USERNAME_WIDTH,
            index: 0,
            value: String::new(),
        }),
        password: Dispatch::from(TextLine {
            pos: PASSWORD_POS,
            display_width: PASSWORD_WIDTH,
            index: 0,
            value: String::new(),
        }),
        pg_user: Dispatch::from(TextLine {
            pos: PG_USER_POS,
            display_width: PG_USER_WIDTH,
            index: 0,
            value: String::new(),
        }),
        pg_password: Dispatch::from(TextLine {
            pos: PG_PASSWORD_POS,
            display_width: PG_PASSWORD_WIDTH,
            index: 0,
            value: String::new(),
        }),
        pg_address: Dispatch::from(TextLine {
            pos: PG_ADDRESS_POS,
            display_width: PG_ADDRESS_WIDTH,
            index: 0,
            value: String::new(),
        }),
        serve_on: Dispatch::from(TextLine {
            pos: SERVE_ON_POS,
            display_width: SERVE_ON_WIDTH,
            index: 0,
            value: String::new(),
        }),
    };
    scene.insert_input(items.remote.clone());
    scene.insert_input(items.username.clone());
    scene.insert_input(items.password.clone());
    scene.insert_input(items.pg_user.clone());
    scene.insert_input(items.pg_password.clone());
    scene.insert_input(items.pg_address.clone());
    scene.insert_input(items.serve_on.clone());

    scene.insert_block(
        "background".into(),
        Rectangle::new(
            (centered_x(REC_WIDTH), REC_Y, 0),
            (REC_WIDTH, REC_HEIGHT),
            true,
            None,
        ),
    )?;
    scene.insert_block(
        "remote-entry-rec".into(),
        Rectangle::new(
            (centered_x(ENTRY_REC_WIDTH), REMOTE_REC_Y, 0),
            (ENTRY_REC_WIDTH, 3),
            true,
            Some(TextLine {
                pos: (0, 0, 0),
                display_width: REMOTE_HEADING_WIDTH,
                index: 0,
                value: REMOTE_HEADING_TEXT.into(),
            }),
        ),
    )?;
    scene.insert_block(
        "username-entry-rec".into(),
        Rectangle::new(
            (centered_x(ENTRY_REC_WIDTH), USERNAME_REC_Y, 0),
            (ENTRY_REC_WIDTH, 3),
            true,
            Some(TextLine {
                pos: (0, 0, 0),
                display_width: USERNAME_HEADING_WIDTH,
                index: 0,
                value: USERNAME_HEADING_TEXT.into(),
            }),
        ),
    )?;
    scene.insert_block(
        "password-entry-rec".into(),
        Rectangle::new(
            (centered_x(ENTRY_REC_WIDTH), PASSWORD_REC_Y, 0),
            (ENTRY_REC_WIDTH, 3),
            true,
            Some(TextLine {
                pos: (0, 0, 0),
                display_width: PASSWORD_HEADING_WIDTH,
                index: 0,
                value: PASSWORD_HEADING_TEXT.into(),
            }),
        ),
    )?;
    scene.insert_input(Button::new(
        (centered_x(LOGIN_WIDTH + 4), LOGIN_Y, 0),
        LOGIN_TEXT.into(),
        LOGIN_WIDTH + 4,
        2,
        Some(|| TerminalCode::None),
    ));

    scene.insert_block(
        "divider".into(),
        LineHorizontal {
            pos: (centered_x(REC_WIDTH), DIVIDER_Y, 0),
            length: REC_WIDTH,
        },
    )?;

    scene.insert_block(
        "pg-user-entry-rec".into(),
        Rectangle::new(
            (POSTGRES_ENTRY_LEFT_X, POSTGRES_ROW_1_Y, 0),
            (POSTGRES_ENTRY_WIDTH, 3),
            true,
            Some(TextLine {
                pos: (0, 0, 0),
                display_width: POSTGRES_USER_WIDTH,
                index: 0,
                value: POSTGRES_USER_TEXT.into(),
            }),
        ),
    )?;
    scene.insert_block(
        "pg-password-entry-rec".into(),
        Rectangle::new(
            (POSTGRES_ENTRY_RIGHT_X, POSTGRES_ROW_1_Y, 0),
            (POSTGRES_ENTRY_WIDTH, 3),
            true,
            Some(TextLine {
                pos: (0, 0, 0),
                display_width: POSTGRES_PASSWORD_WIDTH,
                index: 0,
                value: POSTGRES_PASSWORD_TEXT.into(),
            }),
        ),
    )?;
    scene.insert_block(
        "pg-address-entry-rec".into(),
        Rectangle::new(
            (POSTGRES_ENTRY_LEFT_X, POSTGRES_ROW_2_Y, 0),
            (POSTGRES_ENTRY_WIDTH, 3),
            true,
            Some(TextLine {
                pos: (0, 0, 0),
                display_width: POSTGRES_ADDRESS_WIDTH,
                index: 0,
                value: POSTGRES_ADDRESS_TEXT.into(),
            }),
        ),
    )?;
    scene.insert_block(
        "serve-address-entry-rec".into(),
        Rectangle::new(
            (POSTGRES_ENTRY_RIGHT_X, POSTGRES_ROW_2_Y, 0),
            (POSTGRES_ENTRY_WIDTH, 3),
            true,
            Some(TextLine {
                pos: (0, 0, 0),
                display_width: SERVE_ON_ADDRESS_WIDTH,
                index: 0,
                value: SERVE_ON_ADDRESS_TEXT.into(),
            }),
        ),
    )?;

    scene.insert_input(Button::new(
        (centered_x(HOST_WIDTH + 4), HOST_Y, 0),
        HOST_TEXT.into(),
        HOST_WIDTH + 4,
        2,
        Some(|| TerminalCode::None),
    ));
    Ok((scene, vec![], items))
}

const fn centered_x(width: u16) -> u16 {
    (WIDTH / 2).saturating_sub(width / 2)
}
