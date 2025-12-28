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
                hangul text unique,
                description text,
                constraint hangul_log_to_account
                    foreign key (account_id)
                    references account (account_id)
                    on delete cascade
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

#[derive(Debug, Clone)]
pub struct Account {
    account_id: i64,
    username: String,
    db: Database,
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
        Self::create_tables(&pool).await?;

        Ok(Self { pool })
    }

    pub async fn sign_in(
        self,
        username: &str,
        password: &str,
    ) -> DatabaseResult<SignInAttempt> {
        let row =
            Self::select_account_row(&self.pool, username, password).await?;
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
    ) -> DatabaseResult<SignUpAttempt> {
        if Self::exists_account_username(&self.pool, username).await? {
            return Ok(SignUpAttempt::Failed);
        }
        match Self::insert_account_row(&self.pool, username, password).await {
            Ok(()) => Ok(SignUpAttempt::Success),
            Err(e) => Err(e),
        }
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
}

#[derive(Debug, Clone)]
pub enum SignInAttempt {
    Success(Account),
    Failed(Database),
}

#[derive(Debug, Clone)]
pub enum SignUpAttempt {
    Success,
    Failed,
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

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("{0}")]
    Sqlx(#[from] sqlx::Error),
}
pub type DatabaseResult<T> = Result<T, DatabaseError>;
