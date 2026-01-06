use std::{rc::Rc, sync::RwLock};

use isahc::{Request, RequestExt, http::StatusCode};
use terminal::{
    Scene, TerminalResult,
    code::TerminalCode,
    elements::{Button, Dispatch, LineHorizontal, LineVertical, TextLine},
};

use crate::{
    Client, HttpResult,
    elements::{
        DescriptionInput, HangulResult, JamoInfo, Log, PossibleInfo, RrInput,
    },
    hangul::Hangul,
    host::{InsertEntry, database::Account},
    scenes::error_popup_scene,
};

/* All syllables take 2 columns
             1         2         3         4         5         6         7         8
   012345678901234567890123456789012345678901234567890123456789012345678901234567890
00 +――――――――――――――top-1――――――――――――――――――――+―――――――――――――top-2―――――――――――――――――――――+
01     ------ Info ------                  │       ----  LOG ----
02 +―――――+――――――――info-bot―――――――――――――――――+
03       │  --------Hangul-------          │
04 +―――――+―――――――hangul-bot――――――――――――――――+
05  rr   │                                 │
06 +―――――+―――――――――rr-bot――――――――――――――――――+
07  Desc │                                 │
08 +―――――+―――――――――desc-bot――――――――――――――――+
09                 SAVE                    │
10 +―――――――――――――――save-bot――――――――――――――――+
11  Combinations:                          │
12                                         │
13                                         │
14 +―――――――――――――――comb-bot――――――――――――――――+
15     -------- Jamo Index -----------     │
16                                         │
17                                         │
18                                         │
19                                         │
20                                         │
21                                         │
22                                       mid-v
23                                         │
24                                         │
25                                         │
26                                         │
27                                         │
28                                         │
29                                         │
30                                         +
*/

#[allow(dead_code)]
pub struct MainItems {
    pub hangul_result: Dispatch<HangulResult>,
    pub rr: Dispatch<RrInput>,
    pub description_input: DescriptionInput,
    pub log: Dispatch<Log>,
}

pub fn main_scene(
    full_wh: (u16, u16),
    client: Rc<RwLock<Client>>,
    account: Rc<RwLock<Option<Account>>>,
) -> TerminalResult<(Scene, Vec<(String, Scene)>, MainItems)> {
    let mut scene = Scene::default();
    /*
     * Lines
     */
    {
        scene.insert_block(
            "mid-v".into(),
            LineVertical::default()
                .with_x(40)
                .with_line_start(0)
                .with_length(31)
                .clone(),
        )?;
        scene.insert_block(
            "top-1".into(),
            LineHorizontal::default()
                .with_y(0)
                .with_line_start(0)
                .with_length(41)
                .clone(),
        )?;
        scene.insert_block(
            "top-2".into(),
            LineHorizontal::default()
                .with_y(0)
                .with_line_start(40)
                .with_length(41)
                .clone(),
        )?;
        scene.insert_block(
            "info-bot".into(),
            LineHorizontal::default()
                .with_y(2)
                .with_line_start(0)
                .with_length(41)
                .clone(),
        )?;
        scene.insert_block(
            "hangul-bot".into(),
            LineHorizontal::default()
                .with_y(4)
                .with_line_start(0)
                .with_length(41)
                .clone(),
        )?;
        scene.insert_block(
            "hangul-left".into(),
            LineVertical::default()
                .with_x(8)
                .with_z_index(1)
                .with_line_start(2)
                .with_length(3)
                .clone(),
        )?;
        scene.insert_block(
            "rr-bot".into(),
            LineHorizontal::default()
                .with_y(6)
                .with_line_start(0)
                .with_length(41)
                .clone(),
        )?;
        scene.insert_block(
            "rr-left".into(),
            LineVertical::default()
                .with_x(6)
                .with_line_start(4)
                .with_length(3)
                .clone(),
        )?;
        scene.insert_block(
            "desc-bot".into(),
            LineHorizontal::default()
                .with_y(8)
                .with_line_start(0)
                .with_length(41)
                .clone(),
        )?;
        scene.insert_block(
            "desc-left".into(),
            LineVertical::default()
                .with_x(6)
                .with_z_index(1)
                .with_line_start(6)
                .with_length(3)
                .clone(),
        )?;
        scene.insert_block(
            "save-bot".into(),
            LineHorizontal::default()
                .with_y(10)
                .with_line_start(0)
                .with_length(41)
                .clone(),
        )?;
        scene.insert_block(
            "comb-bot".into(),
            LineHorizontal::default()
                .with_y(14)
                .with_line_start(0)
                .with_length(41)
                .clone(),
        )?;
    }

    /*
     * Info
     */
    {
        const STR: &str = "Exit: ^q   Help: ^h   Menu: ^Space";
        scene.insert_block(
            "info".into(),
            TextLine::default()
                .with_pos(1, 1)
                .with_width(STR.len() as u16)
                .with_value(STR.into())
                .clone(),
        )?;
    }
    /*
     * Hangul
     */
    let hangul_result = {
        scene.insert_block(
            "hangul-text".into(),
            TextLine::default()
                .with_pos(1, 3)
                .with_width(8)
                .with_value("Hangul".into())
                .clone(),
        )?;
        let h = Dispatch::from(HangulResult::new((10, 3, 0)));
        scene.insert_block("hangul".into(), h.clone())?;
        h
    };
    /*
     * rr
     */
    let rr = {
        let rr = Dispatch::from(RrInput::new(
            TextLine::default().with_pos(8, 5).with_width(31).clone(),
            hangul_result.clone(),
        ));
        scene.insert_block(
            "rr-text".into(),
            TextLine::default()
                .with_pos(1, 5)
                .with_width(4)
                .with_value("RR".into())
                .clone(),
        )?;
        scene.insert_input(rr.clone());
        rr
    };
    /*
     * Desc
     */
    let description_input = {
        let d = DescriptionInput::from(
            TextLine::default().with_pos(8, 7).with_width(31),
        );
        scene.insert_block(
            "desc-text".into(),
            TextLine::default()
                .with_pos(1, 7)
                .with_width(4)
                .with_value("Desc".into())
                .clone(),
        )?;
        scene.insert_input(d.clone());
        d
    };
    /*
     * Log
     */
    let entry_log = {
        let l = Dispatch::from(
            Log::new((42, 1, 0), 38, 29, client.clone(), account.clone())?
                .with_input_pos((80, 30))
                .clone(),
        );
        scene.insert_input(l.clone());
        l
    };
    /*
     * SAVE
     */
    {
        let client = client.clone();
        let account = account.clone();
        let rr = rr.clone();
        let di = description_input.clone();
        let b = Button::new(
            (1, 9, 0),
            "SAVE".into(),
            38,
            17,
            Some(move || {
                let mut rr = rr.write().unwrap_or_else(|e| e.into_inner());
                let mut di = di.write().unwrap_or_else(|e| e.into_inner());
                let hangul = rr.hangul();
                let hangul_guard =
                    hangul.read().unwrap_or_else(|e| e.into_inner());
                let hangul = hangul_guard.str();
                let description = di.value();

                if hangul.is_empty() {
                    return TerminalCode::GoToScene(
                        "empty-hangul-error".into(),
                    );
                }

                if description.is_empty() {
                    return TerminalCode::GoToScene(
                        "empty-description-error".into(),
                    );
                }

                let client = client.read().unwrap_or_else(|e| e.into_inner());
                let client = &*client;
                let account = account.read().unwrap_or_else(|e| e.into_inner());
                let account = if let Some(account) = account.as_ref() {
                    account
                } else {
                    return TerminalCode::GoToScene("no-account-error".into());
                };

                let res =
                    insert_entry(&*client, &*account, hangul, description);
                if !matches!(res, Ok(true)) {
                    return TerminalCode::GoToScene(
                        "entry-exists-error".into(),
                    );
                }
                drop(hangul_guard);
                rr.clear();
                di.clear();
                TerminalCode::Reload
            }),
        );
        scene.insert_input(b);
    };
    /*
     * Combinations
     */
    {
        scene.insert_block(
            "combinations".into(),
            PossibleInfo::new((1, 11, 0), hangul_result.clone()),
        )?;
    }
    /*
     * Jamo index
     */
    {
        scene.insert_block("jamo-box".into(), JamoInfo::new((0, 15, 0)))?;
    }

    let empty_hangul_error =
        error_popup_scene(full_wh, "Hangul field is empty!".into(), &[], true)?;
    let empty_description_error = error_popup_scene(
        full_wh,
        "Description field is empty!".into(),
        &[],
        true,
    )?;
    let no_account_error =
        error_popup_scene(full_wh, "Not signed in".into(), &[], true)?;
    let entry_exists_error =
        error_popup_scene(full_wh, "Entry already exists".into(), &[], true)?;

    Ok((
        scene,
        vec![
            ("empty-hangul-error".into(), empty_hangul_error),
            ("empty-description-error".into(), empty_description_error),
            ("no-account-error".into(), no_account_error),
            ("entry-exists-error".into(), entry_exists_error),
        ],
        MainItems {
            hangul_result,
            rr,
            description_input,
            log: entry_log,
        },
    ))
}

// true if insertion was succesful
// and reload is required
fn insert_entry(
    client: &Client,
    account: &Account,
    hangul: &Hangul,
    description: &str,
) -> HttpResult<bool> {
    Ok(Request::post(format!("{}/log/insert", client.remote))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&InsertEntry {
            account: account.clone(),
            hangul: hangul.to_string(),
            description: description.to_string(),
        })?)?
        .send()?
        .status()
        == StatusCode::RESET_CONTENT)
}
