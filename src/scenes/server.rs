use std::{rc::Rc, sync::RwLock, u16};

use terminal::{
    Scene, SceneType, TerminalResult,
    code::TerminalCode,
    event::KeyEvent,
    traits::{Block, Input},
};
use tokio::task::JoinHandle;

use crate::host::{Host, serve};

#[allow(dead_code)]
pub struct ServerItems {}

pub fn server_scene(
    host: Rc<RwLock<Host>>,
    host_running: Rc<RwLock<bool>>,
) -> TerminalResult<(Scene, Vec<(String, Scene)>, ServerItems)> {
    let mut scene = Scene::new(SceneType::FullNoClear);
    scene.insert_input(HostServe::new(host.clone(), host_running));

    let items = ServerItems {};

    Ok((scene, vec![], items))
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

    fn load(&mut self) {
        log::debug!("load");
        match self.host_running.read() {
            Ok(guard) if !*guard => return,
            Err(_) => return,
            _ => (),
        };
        let host = self.host.read().unwrap_or_else(|e| e.into_inner()).clone();
        self.thread = Some(tokio::spawn(async move {
            log::debug!("Started server");
            match serve(&host).await {
                Ok(_) => (),
                Err(e) => log::error!("{}", e),
            }
        }));
    }

    fn unload(&mut self) {
        self.thread.as_mut().map(|t| t.abort());
        self.thread = None
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
}
