pub mod greeting_command;
pub mod greeting_query;

use sqlx::migrate::MigrateError;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, Pool, Postgres};
use log::{error, info};

pub async fn init_db(db_url: String) -> Result<Pool<sqlx::Postgres>, DbError> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&*db_url)
        .await?;
    Ok(pool)
}

pub async fn migrate(pool: Pool<Postgres>) -> Result<(), DbError> {
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(())
}


pub async fn generate_logg(pool: Pool<sqlx::Postgres>) {
    loop {
        // tokio::time::sleep(Duration::from_secs(5)).await;

        match pool.begin().await {
            Err(e) => error!("{}", e),
            Ok(mut transaction) => {
                sqlx::query(
                    "do
                        $$
                            begin
                                perform public.generate_logg();
                            end
                        $$;",
                )
                    .execute(&mut *transaction)
                    .await
                    .expect("Failed executing statement");
                info!("Generating log");
                transaction.commit().await.expect("");
            }
        }
    }
}
#[derive(Debug)]
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
