// #![allow(dead_code)]

use terminal::{Terminal, TerminalResult, code::TerminalCode};

use crate::scenes::{MainItems, help_menu_scene, main_scene, menu_scene};

mod elements;
mod ext;
mod hangul;
mod hangul_parser;
mod jamo;
mod scenes;
mod syllable;

use terminal::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

macro_rules! esc {
    () => {
        KeyEvent {
            code: KeyCode::Esc,
            kind: KeyEventKind::Press,
            ..
        }
    };
}
macro_rules! ctrl {
    ($c:expr) => {
        KeyEvent {
            code: KeyCode::Char($c),
            modifiers: KeyModifiers::CONTROL,
            ..
        }
    };
}

// Assuming char is 1:2
// 4:3 becomes 8:3
fn main() -> TerminalResult<()> {
    pretty_env_logger::init();
    let (main_scene, scenes, MainItems { log, .. }) = main_scene((81, 31))?;
    let mut term =
        Terminal::with_key_listener("main".into(), main_scene, |k| match k {
            esc!() => TerminalCode::PreviousScene,
            ctrl!('h') => TerminalCode::GoToScene("help".into()),
            ctrl!(' ') => TerminalCode::GoToScene("menu".into()),
            _ => TerminalCode::UnhandledKey(k),
        });

    for (name, scene) in scenes {
        term.insert_scene(name, scene);
    }
    term.insert_scene("help".into(), help_menu_scene()?);

    let (menu_scene, scenes) = menu_scene((81, 31), log)?;
    term.insert_scene("menu".into(), menu_scene);
    for (name, scene) in scenes {
        term.insert_scene(name, scene);
    }

    term.run((81, 31))
}
