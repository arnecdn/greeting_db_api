pub mod greeting_command;
pub mod greeting_query;


use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, Pool};
use sqlx::migrate::MigrateError;

pub async fn init_db(db_url: String) -> Result<Pool<sqlx::Postgres>, DbError> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&*db_url)
        .await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}

#[derive(Debug)]
pub struct  DbError{
    pub error_message: String
}
impl From<MigrateError> for DbError {
    fn from(value:  MigrateError) -> Self {
        DbError{error_message:value.to_string()}
    }
}

impl From<Error> for DbError {
    fn from(value: Error) -> Self {
        DbError{error_message:value.to_string()}
    }
}

impl From<&str> for DbError {
    fn from(value: &str) -> Self {
        DbError { error_message: value.to_string() }
    }
}