use std::fmt::{self};

use sqlx::{
    Executor, FromRow, PgPool, Postgres,
    postgres::{PgDatabaseError, PgPoolOptions},
};

macro_rules! create_table_account {
    ($executor:expr) => {
        sqlx::query(
            "create table if not exists account (
                account_id bigserial primary key,
                username varchar(30) unique,
                password varchar(30)
            )",
        )
        .execute($executor)
    };
}
macro_rules! create_table_hangul_log {
    ($executor:expr) => {
        sqlx::query(
            "create table if not exists hangul_log (
                hangul_log_id bigserial primary key,
                account_id bigint not null,
                hangul text,
                description text,
                constraint hangul_log_to_account
                    foreign key (account_id)
                    references account (account_id)
                    on delete cascade,
                constraint hangul_log_hangul_unique_with_account
                    unique (account_id, hangul)
            )",
        )
        .execute($executor)
    };
}
macro_rules! select_account_row {
    ($executor:expr, $username:expr, $password:expr) => {
        sqlx::query_as(&format!(
            "select account_id, username
                from account
                where username = '{}'
                    and password = '{}'
            ",
            $username, $password
        ))
        .fetch_optional($executor)
    };
}
macro_rules! exists_account_username {
    ($executor:expr, $username:expr) => {
        sqlx::query_as(&format!(
            "select exists (
                select 1
                from account
                where username = '{}'
            ) as exists",
            $username
        ))
        .fetch_one($executor)
    };
}
macro_rules! insert_account_row {
    ($executor:expr, $username:expr, $password:expr) => {
        sqlx::query(&format!(
            "insert into account (username, password)
                values ('{}', '{}')
            ",
            $username, $password
        ))
        .execute($executor)
    };
}
macro_rules! select_account_id_hangul_log {
    ($executor:expr, $account_id:expr) => {
        sqlx::query_as(&format!(
            "select hangul_log_id, hangul, description
                from hangul_log
                where account_id = {}
            ",
            $account_id
        ))
        .fetch_all($executor)
    };
}
macro_rules! exists_hangul_log_account_hangul {
    ($executor:expr, $account_id:expr, $hangul:expr) => {
        sqlx::query_as(&format!(
            "select exists (
                select 1
                from hangul_log
                where account_id = {}
                    and hangul = '{}'
            ) as exists",
            $account_id, $hangul
        ))
        .fetch_one($executor)
    };
}
macro_rules! insert_hangul_log_row {
    ($executor:expr, $account_id:expr, $hangul:expr, $description:expr) => {
        sqlx::query(&format!(
            "insert into hangul_log (account_id, hangul, description)
                values ({}, '{}', '{}')
            ",
            $account_id, $hangul, $description
        ))
        .execute($executor)
    };
}
macro_rules! delete_hangul_log_row {
    ($executor:expr, $account_id:expr, $hangul:expr) => {
        sqlx::query(&format!(
            "delete from hangul_log
                where account_id = {}
                    and hangul = '{}'
            ",
            $account_id, $hangul
        ))
        .execute($executor)
    };
}

#[derive(Debug, Clone)]
pub struct Account {
    account_id: i64,
    username: String,
    db: Database,
}

impl Account {
    pub fn username(&self) -> &str {
        &self.username
    }

    pub async fn all_rows(&self) -> DatabaseResult<Vec<HangulLogRow>> {
        select_account_id_hangul_log(&self.db.pool, self.account_id)
            .await
            .map_err(DatabaseError::from)
    }

    pub async fn insert_row(
        &self,
        hangul: &str,
        description: &str,
    ) -> DatabaseResult<bool> {
        if exists_hangul_log_account_hangul(
            &self.db.pool,
            self.account_id,
            hangul,
        )
        .await?
        {
            return Ok(false);
        }
        match insert_hangul_log_row(
            &self.db.pool,
            self.account_id,
            hangul,
            description,
        )
        .await
        {
            Ok(()) => Ok(true),
            Err(e) => Err(e),
        }
    }

    pub async fn delete_row(&self, hangul: &str) -> DatabaseResult<bool> {
        if !exists_hangul_log_account_hangul(
            &self.db.pool,
            self.account_id,
            hangul,
        )
        .await?
        {
            return Ok(false);
        }
        match delete_hangul_log_row(&self.db.pool, self.account_id, hangul)
            .await
        {
            Ok(()) => Ok(true),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(
        user: &str,
        password: &str,
        ip: &str,
    ) -> DatabaseResult<Self> {
        let url = "postgres://"
            .chars()
            .chain(user.chars())
            .chain(Some(':'))
            .chain(password.chars())
            .chain(Some('@'))
            .chain(ip.chars())
            .chain("/langlog".chars())
            .collect::<String>();
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&url)
            .await?;
        create_tables(&pool).await?;

        Ok(Self { pool })
    }

    pub async fn sign_in(
        self,
        username: &str,
        password: &str,
    ) -> DatabaseResult<SignInAttempt> {
        let row = select_account_row(&self.pool, username, password).await?;
        Ok(match row {
            Some(AccountRow {
                account_id,
                username,
            }) => SignInAttempt::Success(Account {
                account_id,
                username,
                db: self,
            }),
            None => SignInAttempt::Failed(self),
        })
    }

    pub async fn sign_up(
        &self,
        username: &str,
        password: &str,
    ) -> DatabaseResult<bool> {
        if exists_account_username(&self.pool, username).await? {
            return Ok(false);
        }
        match insert_account_row(&self.pool, username, password).await {
            Ok(()) => Ok(true),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct HangulLogRow {
    hangul_log_id: i64,
    hangul: String,
    description: String,
}
impl HangulLogRow {
    pub fn hangul(&self) -> &str {
        &self.hangul
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

#[derive(Debug, Clone)]
pub enum SignInAttempt {
    Success(Account),
    Failed(Database),
}

#[derive(Debug, Clone, sqlx::FromRow)]
struct AccountRow {
    account_id: i64,
    username: String,
}

#[derive(Debug, Clone, Copy, sqlx::FromRow)]
struct Exists {
    exists: bool,
}

async fn create_tables<'a, E>(executor: E) -> DatabaseResult<()>
where
    E: Executor<'a, Database = Postgres> + Copy,
{
    create_table_account!(executor).await?;
    create_table_hangul_log!(executor).await?;
    Ok(())
}

async fn select_account_row<'a, E, U, P>(
    executor: E,
    username: U,
    password: P,
) -> DatabaseResult<Option<AccountRow>>
where
    E: Executor<'a, Database = Postgres> + Copy,
    U: fmt::Display,
    P: fmt::Display,
{
    select_account_row!(executor, username, password)
        .await
        .map_err(DatabaseError::from)
}

async fn exists_account_username<'a, E, U>(
    executor: E,
    username: U,
) -> DatabaseResult<bool>
where
    E: Executor<'a, Database = Postgres> + Copy,
    U: fmt::Display,
{
    let Exists { exists } =
        exists_account_username!(executor, username).await?;
    Ok(exists)
}

async fn insert_account_row<'a, E, U, P>(
    executor: E,
    username: U,
    password: P,
) -> DatabaseResult<()>
where
    E: Executor<'a, Database = Postgres> + Copy,
    U: fmt::Display,
    P: fmt::Display,
{
    insert_account_row!(executor, username, password).await?;
    Ok(())
}

async fn select_account_id_hangul_log<'a, E>(
    executor: E,
    account_id: i64,
) -> DatabaseResult<Vec<HangulLogRow>>
where
    E: Executor<'a, Database = Postgres> + Copy,
{
    select_account_id_hangul_log!(executor, account_id)
        .await
        .map_err(DatabaseError::from)
}

async fn exists_hangul_log_account_hangul<'a, E, H>(
    executor: E,
    account_id: i64,
    hangul: H,
) -> DatabaseResult<bool>
where
    E: Executor<'a, Database = Postgres> + Copy,
    H: fmt::Display,
{
    let Exists { exists } =
        exists_hangul_log_account_hangul!(executor, account_id, hangul).await?;
    Ok(exists)
}

async fn insert_hangul_log_row<'a, E, H, D>(
    executor: E,
    account_id: i64,
    hangul: H,
    description: D,
) -> DatabaseResult<()>
where
    E: Executor<'a, Database = Postgres> + Copy,
    H: fmt::Display,
    D: fmt::Display,
{
    insert_hangul_log_row!(executor, account_id, hangul, description).await?;
    Ok(())
}
async fn delete_hangul_log_row<'a, E, H>(
    executor: E,
    account_id: i64,
    hangul: H,
) -> DatabaseResult<()>
where
    E: Executor<'a, Database = Postgres> + Copy,
    H: fmt::Display,
{
    delete_hangul_log_row!(executor, account_id, hangul).await?;
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("{0}")]
    Sqlx(#[from] sqlx::Error),
}
pub type DatabaseResult<T> = Result<T, DatabaseError>;
