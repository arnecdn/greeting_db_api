pub mod greeting_command;
pub mod greeting_query;
pub mod greeting_pg_trace;

use derive_more::with_trait::Display;
use sqlx::migrate::MigrateError;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, Pool, Postgres};
use crate::greeting_pg_trace::PgTraceContext;

pub async fn init_db(db_url: String) -> Result<Pool<sqlx::Postgres>, DbError> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&*db_url)
        .await?;
    Ok(pool)
}

pub async fn migrate(pool: &Pool<Postgres>) -> Result<(), DbError> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}

pub async fn generate_logg(pool: &Box<Pool<Postgres>>, trace: PgTraceContext) -> Result<(), DbError>{
    let mut transaction = pool.begin().await?;

    sqlx::query(&trace.to_sql())
        .execute(&mut *transaction)
        .await?;

    sqlx::query(
        "do
            $$
                begin
                    perform public.generate_logg();
                end
            $$;",
    )
        .execute(&mut *transaction)
        .await?;

    transaction.commit().await?;

    Ok(())
}

#[derive(Display, Debug)]
pub struct DbError {
    pub error_message: String,
}
impl From<MigrateError> for DbError {
    fn from(value: MigrateError) -> Self {
        DbError {
            error_message: value.to_string(),
        }
    }
}

impl From<Error> for DbError {
    fn from(value: Error) -> Self {
        DbError {
            error_message: value.to_string(),
        }
    }
}

impl From<&str> for DbError {
    fn from(value: &str) -> Self {
        DbError {
            error_message: value.to_string(),
        }
    }
}

