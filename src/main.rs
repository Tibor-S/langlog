use std::{
    env,
    fmt::{self},
    rc::Rc,
    sync::RwLock,
};

use terminal::{Terminal, TerminalError, code::TerminalCode};

use crate::{
    host::{Host, HostError},
    scenes::{
        MainItems, help_menu_scene, main_scene, menu_scene, server_scene,
        setup_scene,
    },
};

mod elements;
mod ext;
mod hangul;
mod hangul_parser;
mod host;
mod jamo;
mod scenes;
mod syllable;

use terminal::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

macro_rules! return_err {
    ($res:expr) => {
        match $res {
            Ok(v) => v,
            Err(e) => {
                log::error!("{}", e);
                return Err(e.into());
            }
        }
    };
}
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

#[derive(Debug, Default, Clone)]
pub struct Client {
    remote: String,
    username: String,
    password: String,
}

#[tokio::main]
async fn main() -> LanglogResult<()> {
    pretty_env_logger::init();
    let arguments = return_err!(Arguments::new());
    let arg_host = arguments.host.is_some();
    let host_running = Rc::new(RwLock::new(arg_host));
    let host = Rc::new(RwLock::new(arguments.host.unwrap_or_default()));
    let client = Rc::new(RwLock::new(Client::default()));
    let account = Rc::new(RwLock::new(None));
    if arg_host {
        let (server_scene, _, _) =
            return_err!(server_scene(host.clone(), host_running.clone()));
        let mut term = Terminal::new(
            "sever".into(),
            server_scene,
            |k| match k {
                _ => TerminalCode::UnhandledKey(k),
            },
            move || Ok(()),
        );
        return_err!(term.run(None));
        return Ok(());
    }

    // Assuming char is 1:2
    // 4:3 becomes 8:3
    let (main_scene, scenes, MainItems { log, .. }) =
        return_err!(main_scene((81, 31), client.clone(), account.clone()));
    let (setup_scene, _, _) = return_err!(setup_scene(
        client.clone(),
        host.clone(),
        host_running.clone()
    ));
    let (server_scene, _, _) =
        return_err!(server_scene(host.clone(), host_running.clone()));
    let mut term = Terminal::new(
        "setup".into(),
        setup_scene,
        |k| match k {
            esc!() => TerminalCode::PreviousScene,
            ctrl!('h') => TerminalCode::GoToScene("help".into()),
            ctrl!(' ') => TerminalCode::GoToScene("menu".into()),
            _ => TerminalCode::UnhandledKey(k),
        },
        move || Ok(()),
    );

    term.insert_scene("main".into(), main_scene);
    term.insert_scene("server".into(), server_scene);

    for (name, scene) in scenes {
        term.insert_scene(name, scene);
    }
    term.insert_scene("help".into(), return_err!(help_menu_scene()));

    let (menu_scene, scenes) =
        return_err!(menu_scene((81, 31), log, client.clone(), account.clone()));
    term.insert_scene("menu".into(), menu_scene);
    for (name, scene) in scenes {
        term.insert_scene(name, scene);
    }

    return_err!(term.run(Some((81, 31))));
    Ok(())
}

#[derive(Debug, Clone, Default)]
struct Arguments {
    #[allow(dead_code)]
    pub path: String,
    pub host: Option<Host>,
}
impl Arguments {
    pub fn new() -> LanglogResult<Self> {
        let mut builder = ArgumentsBuilder::default();
        builder.read_args()?;
        builder.build()
    }
}

#[derive(Debug, Default)]
struct ArgumentsBuilder {
    is_host: bool,
    user: Option<String>,
    password: Option<String>,
    database: Option<String>,
    listening: Option<String>,
    path: String,
}
impl ArgumentsBuilder {
    pub fn build(self) -> LanglogResult<Arguments> {
        Ok(Arguments {
            path: self.path,
            host: Self::build_host(
                self.is_host,
                self.user,
                self.password,
                self.database,
                self.listening,
            )?,
        })
    }

    fn read_args(&mut self) -> LanglogResult<&mut Self> {
        env::args().take(1).for_each(|arg| self.path = arg);
        env::args()
            .skip(1)
            .map(|arg| self.apply_arg(&Self::parse_arg(&arg)?))
            .collect::<LanglogResult<()>>()?;
        Ok(self)
    }

    fn apply_arg(&mut self, arg: &Arg) -> LanglogResult<()> {
        match arg.as_ref() {
            ArgRef::Short('h') => self.is_host = true,
            ArgRef::Long("user", Some(u)) => self.user = Some(u.to_string()),
            ArgRef::Long("password", Some(p)) => {
                self.password = Some(p.to_string())
            }
            ArgRef::Long("database", Some(d)) => {
                self.database = Some(d.to_string())
            }
            ArgRef::Long("listening", Some(l)) => {
                self.listening = Some(l.to_string())
            }
            _ => {
                return Err(LanglogError::UnknownArgument(arg.clone()));
            }
        };
        Ok(())
    }

    fn build_host(
        is_host: bool,
        user: Option<String>,
        password: Option<String>,
        database: Option<String>,
        listening: Option<String>,
    ) -> LanglogResult<Option<Host>> {
        match (is_host, user, password, database, listening) {
            (false, _, _, _, _) => Ok(None),
            (true, Some(user), Some(password), database, listening) => {
                Ok(Some(Host {
                    pg_user: user,
                    pg_password: password,
                    database_ip: database.unwrap_or("localhost".into()),
                    listening_ip: listening.unwrap_or("localhost:8080".into()),
                }))
            }
            (true, None, _, _, _) => Err(LanglogError::MissingUser),
            (true, _, None, _, _) => Err(LanglogError::MissingPassword),
        }
    }

    fn parse_arg(arg: &str) -> LanglogResult<Arg> {
        match (
            arg.chars().nth(0),
            arg.chars().nth(1),
            arg.chars().nth(2),
            arg.find('='),
        ) {
            (Some('-'), Some(c), None, None) => Ok(Arg::Short(c)),
            (Some('-'), Some('-'), Some('='), _) => {
                Err(LanglogError::InvalidArgument(arg.to_string()))
            }
            (Some('-'), Some('-'), Some(_), None) => {
                Ok(Arg::Long(arg[2..].to_string(), None))
            }
            (Some('-'), Some('-'), Some(_), Some(i)) if i < arg.len() - 1 => {
                Ok(Arg::Long(
                    arg[2..i].to_string(),
                    Some(arg[(i + 1)..].to_string()),
                ))
            }
            _ => Err(LanglogError::InvalidArgument(arg.to_string())),
        }
    }
}
#[derive(Debug, Clone)]
pub enum Arg {
    // -<a>
    Short(char),
    // --<arg>[=<val>]
    Long(String, Option<String>),
}
#[derive(Debug, Clone)]
enum ArgRef<'a> {
    // -<a>
    Short(char),
    // --<arg>[=<val>]
    Long(&'a str, Option<&'a str>),
}
impl Arg {
    fn as_ref<'a>(&'a self) -> ArgRef<'a> {
        match self {
            Arg::Short(c) => ArgRef::Short(*c),
            Arg::Long(sa, None) => ArgRef::Long(sa.as_str(), None),
            Arg::Long(sa, Some(sv)) => {
                ArgRef::Long(sa.as_str(), Some(sv.as_str()))
            }
        }
    }
}
impl fmt::Display for Arg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Arg::Short(c) => write!(f, "-{}", c)?,
            Arg::Long(arg, None) => write!(f, "--{}", arg)?,
            Arg::Long(arg, Some(val)) => write!(f, "--{}={}", arg, val)?,
        };
        Ok(())
    }
}
#[derive(Debug, thiserror::Error)]
pub enum LanglogError {
    #[error("Using -h but no --user was provided")]
    MissingUser,
    #[error("Using -h but no --password was provided")]
    MissingPassword,
    #[error("Unknown argument: {0}")]
    UnknownArgument(Arg),
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
    #[error("{0}")]
    Terminal(#[from] TerminalError),
    #[error("{0}")]
    Host(#[from] HostError),
}
pub type LanglogResult<T> = Result<T, LanglogError>;

#[derive(Debug, thiserror::Error)]
enum HttpError {
    #[error("{0}")]
    Isahc(#[from] isahc::Error),
    #[error("{0}")]
    Http(#[from] isahc::http::Error),
    #[error("{0}")]
    SerdeJson(#[from] serde_json::Error),
}

type HttpResult<T> = Result<T, HttpError>;
