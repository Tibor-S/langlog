use std::{rc::Rc, sync::RwLock, u16};

use terminal::{
    Scene, SceneType, TerminalResult,
    code::TerminalCode,
    elements::{Button, Dispatch, LineHorizontal, Rectangle, TextLine},
    event::KeyEvent,
    ext::call_unary,
    traits::{Block, Input},
};
use tokio::task::JoinHandle;

use crate::{
    elements::{HangulResult, Log, RrInput},
    host::{Host, HostError, serve},
    scenes::error_popup_scene,
};

const WIDTH: u16 = 81;
const HEIGHT: u16 = 31;
const REC_Y: u16 = 2;
const REC_WIDTH: u16 = 61;
const REC_HEIGHT: u16 = 28;

#[allow(dead_code)]
pub struct ServerItems {}

pub fn server_scene(
    host: Rc<RwLock<Host>>,
    host_running: Rc<RwLock<bool>>,
) -> TerminalResult<(Scene, Vec<(String, Scene)>, ServerItems)> {
    let mut scene = Scene::new(SceneType::Full);
    scene.insert_input(HostServe::new(host.clone(), host_running));

    let items = ServerItems {};
    const X: u16 = 10;
    scene.insert_block(
        "database".into(),
        HostText::new((X, 14, 0), host.clone(), |h| {
            format!(
                "Connected to database at: postgres://{}@{}",
                h.pg_user, h.database_ip
            )
        }),
    )?;
    scene.insert_block(
        "listen".into(),
        HostText::new((X, 15, 0), host.clone(), |h| {
            format!("Listening on: {}", h.listening_ip)
        }),
    )?;
    scene.insert_block(
        "listenening-on".into(),
        TextLine {
            pos: (X, 16, 0),
            display_width: "Exit with ^q".len() as u16,
            index: 0,
            value: "Exit with ^q".into(),
        },
    )?;

    Ok((scene, vec![], items))
}

struct HostText<F> {
    pos: (u16, u16, u16),
    host: Rc<RwLock<Host>>,
    f: F,
}
impl<F: Fn(&Host) -> String> HostText<F> {
    fn new(pos: (u16, u16, u16), host: Rc<RwLock<Host>>, f: F) -> Self {
        Self { pos, host, f }
    }
}
impl<F: Fn(&Host) -> String> Block for HostText<F> {
    fn pos(&self) -> (u16, u16, u16) {
        self.pos
    }

    fn rel_line(&self, i: u16) -> Option<String> {
        let text = call_unary(
            &self.f,
            &*self.host.read().unwrap_or_else(|e| e.into_inner()),
        );
        let text_line = TextLine {
            pos: self.pos,
            display_width: text.chars().count() as u16,
            index: 0,
            value: text,
        };
        text_line.rel_line(i)
    }
}

struct HostServe {
    host: Rc<RwLock<Host>>,
    host_running: Rc<RwLock<bool>>,
    thread: Option<JoinHandle<()>>,
}
impl HostServe {
    pub fn new(host: Rc<RwLock<Host>>, host_running: Rc<RwLock<bool>>) -> Self {
        Self {
            host,
            host_running,
            thread: None,
        }
    }
}
impl Block for HostServe {
    fn pos(&self) -> (u16, u16, u16) {
        Default::default()
    }

    fn rel_line(&self, i: u16) -> Option<String> {
        let _ = i;
        None
    }
}
impl Input for HostServe {
    fn feed(&mut self, key: KeyEvent) -> TerminalCode {
        TerminalCode::UnhandledKey(key)
    }

    fn rel_cursor_pos(&self) -> Option<(u16, u16)> {
        None
    }

    fn input_pos(&self) -> (u16, u16) {
        Default::default()
    }

    fn focus(&mut self) {
        match self.host_running.read() {
            Ok(guard) if !*guard => return,
            Err(_) => return,
            _ => (),
        };
        let host = self.host.read().unwrap_or_else(|e| e.into_inner()).clone();
        self.thread = Some(tokio::spawn(async move {
            match serve(&host).await {
                Ok(_) => (),
                Err(e) => log::error!("{}", e),
            }
        }));
    }

    fn unfocus(&mut self) {
        self.thread.as_mut().map(|t| t.abort());
        self.thread = None
    }
}

const fn centered_x(width: u16) -> u16 {
    (WIDTH / 2).saturating_sub(width / 2)
}
