// #![allow(dead_code)]

use std::{
    env,
    fmt::{self, Write, format},
    usize,
};

use terminal::{Terminal, TerminalError, TerminalResult, code::TerminalCode};

use crate::{
    database::{Database, DatabaseError, SignInAttempt},
    scenes::{MainItems, help_menu_scene, main_scene, menu_scene},
};

mod database;
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

#[tokio::main]
async fn main() -> LanglogResult<()> {
    pretty_env_logger::init();
    let arguments = Arguments::new()?;
    let host = arguments.host.unwrap();

    let db = Database::new(&host.user, &host.password, &host.ip).await?;
    log::debug!("{:?}", db.sign_up("frisco", "1234").await?);
    let account = match db.sign_in("frisco", "1234").await? {
        SignInAttempt::Success(a) => a,
        SignInAttempt::Failed(db) => panic!("lol"),
    };
    log::debug!("{:?}", account);
    for e in account.all_rows().await? {
        log::debug!("{}: {}", e.hangul(), e.description())
    }
    account.insert_row("ne", "yes").await?;
    for e in account.all_rows().await? {
        log::debug!("{}: {}", e.hangul(), e.description())
    }
    account.delete_row("ne").await?;
    for e in account.all_rows().await? {
        log::debug!("{}: {}", e.hangul(), e.description())
    }

    // Assuming char is 1:2
    // 4:3 becomes 8:3
    let (main_scene, scenes, MainItems { log, .. }) = main_scene((81, 31))?;
    let main_log = log.clone();
    let mut term = Terminal::new(
        "main".into(),
        main_scene,
        |k| match k {
            esc!() => TerminalCode::PreviousScene,
            ctrl!('h') => TerminalCode::GoToScene("help".into()),
            ctrl!(' ') => TerminalCode::GoToScene("menu".into()),
            _ => TerminalCode::UnhandledKey(k),
        },
        move || {
            main_log.read().unwrap().save()?;
            Ok(())
        },
    );

    for (name, scene) in scenes {
        term.insert_scene(name, scene);
    }
    term.insert_scene("help".into(), help_menu_scene()?);

    let (menu_scene, scenes) = menu_scene((81, 31), log)?;
    term.insert_scene("menu".into(), menu_scene);
    for (name, scene) in scenes {
        term.insert_scene(name, scene);
    }

    term.run((81, 31)).map_err(LanglogError::from)
}

#[derive(Debug, Clone, Default)]
struct Arguments {
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
    ip: Option<String>,
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
                self.ip,
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
            ArgRef::Long("ip", Some(i)) => self.ip = Some(i.to_string()),
            _ => {
                return Err(LanglogError::UnknownArgument(format!("{}", arg)));
            }
        };
        Ok(())
    }

    fn build_host(
        is_host: bool,
        user: Option<String>,
        password: Option<String>,
        ip: Option<String>,
    ) -> LanglogResult<Option<Host>> {
        match (is_host, user, password, ip) {
            (false, _, _, _) => Ok(None),
            (true, Some(user), Some(password), Some(ip)) => {
                Ok(Some(Host { user, password, ip }))
            }
            (true, None, _, _) => Err(LanglogError::MissingUser),
            (true, _, None, _) => Err(LanglogError::MissingPassword),
            (true, _, _, None) => Err(LanglogError::MissingIp),
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
enum Arg {
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
#[derive(Debug, Clone)]
struct Host {
    pub user: String,
    pub password: String,
    pub ip: String,
}

#[derive(Debug, thiserror::Error)]
pub enum LanglogError {
    #[error("Using -h but no --user was provided")]
    MissingUser,
    #[error("Using -h but no --password was provided")]
    MissingPassword,
    #[error("Using -h but no --ip was provided")]
    MissingIp,
    #[error("Unknown argument: {0}")]
    UnknownArgument(String),
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
    #[error("{0}")]
    Terminal(#[from] TerminalError),
    #[error("{0}")]
    Database(#[from] DatabaseError),
}
pub type LanglogResult<T> = Result<T, LanglogError>;
