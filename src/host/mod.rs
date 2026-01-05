use std::{io, sync::Arc};

use axum::{
    Json, Router,
    http::StatusCode,
    routing::{delete, get, post},
};
use serde::Deserialize;

use crate::host::database::{Account, Database, DatabaseError, HangulLogRow};

pub mod database;

#[derive(Debug, Clone, Default)]
pub struct Host {
    pub pg_user: String,
    pub pg_password: String,
    pub database_ip: String,
    pub listening_ip: String,
}

pub async fn serve(host: &Host) -> HostResult<()> {
    let database = Arc::new(
        Database::new(&host.pg_user, &host.pg_password, &host.database_ip)
            .await?,
    );
    log::info!(
        "Connected to database: {}@{}",
        &host.pg_user,
        &host.database_ip
    );

    let listener = tokio::net::TcpListener::bind(&host.listening_ip).await?;
    log::info!("Listening on {}", &host.listening_ip);

    let app = Router::new()
        .route(
            "/signin",
            get({
                let db = Arc::clone(&database);
                move |body| sign_in(body, db)
            }),
        )
        .route(
            "/signup",
            post({
                let db = Arc::clone(&database);
                move |body| sign_up(body, db)
            }),
        )
        .route(
            "/log",
            get({
                let db = Arc::clone(&database);
                move |body| log_all_entries(body, db)
            }),
        )
        .route(
            "/log/insert",
            post({
                let db = Arc::clone(&database);
                move |body| log_insert_entry(body, db)
            }),
        )
        .route(
            "/log/delete",
            delete({
                let db = Arc::clone(&database);
                move |body| log_delete_entry(body, db)
            }),
        );
    axum::serve(listener, app).await.map_err(HostError::from)
}

async fn sign_in(
    Json(SignIn { username, password }): Json<SignIn>,
    db: Arc<Database>,
) -> (StatusCode, Json<Option<Account>>) {
    match db
        .sign_in(&username, &password)
        .await
        .map_err(HostError::from)
    {
        Ok(Some(account)) => (StatusCode::OK, Json::from(Some(account))),
        Ok(None) => (StatusCode::UNAUTHORIZED, Default::default()),
        Err(e) => e.into(),
    }
}

async fn sign_up(
    Json(SignIn { username, password }): Json<SignIn>,
    db: Arc<Database>,
) -> (StatusCode, Json<Option<Account>>) {
    match db
        .sign_up(&username, &password)
        .await
        .map_err(HostError::from)
    {
        Ok(true) => (),
        Ok(false) => return (StatusCode::UNAUTHORIZED, Default::default()),
        Err(e) => return e.into(),
    }
    match db
        .sign_in(&username, &password)
        .await
        .map_err(HostError::from)
    {
        Ok(Some(account)) => (StatusCode::OK, Json::from(Some(account))),
        Ok(None) => (StatusCode::UNAUTHORIZED, Default::default()),
        Err(e) => e.into(),
    }
}

async fn log_all_entries(
    Json(account): Json<Account>,
    db: Arc<Database>,
) -> (StatusCode, Json<Vec<HangulLogRow>>) {
    match db.log_all_rows(&account).await.map_err(HostError::from) {
        Ok(v) => (StatusCode::OK, Json::from(v)),
        Err(e) => e.into(),
    }
}

async fn log_insert_entry(
    Json(InsertEntry {
        account,
        hangul,
        description,
    }): Json<InsertEntry>,
    db: Arc<Database>,
) -> StatusCode {
    match db
        .log_insert_row(&account, &hangul, &description)
        .await
        .map_err(HostError::from)
    {
        Ok(true) => StatusCode::RESET_CONTENT,
        Ok(false) => StatusCode::UNPROCESSABLE_ENTITY,
        Err(e) => e.into(),
    }
}

async fn log_delete_entry(
    Json(DeleteEntry { account, hangul }): Json<DeleteEntry>,
    db: Arc<Database>,
) -> StatusCode {
    match db
        .log_delete_row(&account, &hangul)
        .await
        .map_err(HostError::from)
    {
        Ok(true) => StatusCode::RESET_CONTENT,
        Ok(false) => StatusCode::UNPROCESSABLE_ENTITY,
        Err(e) => e.into(),
    }
}

#[derive(Deserialize)]
struct SignIn {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct InsertEntry {
    account: Account,
    hangul: String,
    description: String,
}

#[derive(Deserialize)]
struct DeleteEntry {
    account: Account,
    hangul: String,
}

#[derive(Debug, thiserror::Error)]
pub enum HostError {
    #[error("{0}")]
    Database(#[from] DatabaseError),
    #[error("{0}")]
    Io(#[from] io::Error),
}
pub type HostResult<T> = Result<T, HostError>;
impl<T: Default> From<HostError> for (StatusCode, Json<T>) {
    fn from(value: HostError) -> Self {
        log::error!("{}", value);
        (StatusCode::INTERNAL_SERVER_ERROR, Default::default())
    }
}
impl From<HostError> for StatusCode {
    fn from(value: HostError) -> Self {
        log::error!("{}", value);
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
