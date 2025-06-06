use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{Error, Pool};
use uuid::Uuid;
use crate::DbError;

pub struct GreetingCommandRepositoryImpl {
    pool: Box<Pool<sqlx::Postgres>>,
}

#[async_trait]
pub trait GreetingCommandRepository {
    async fn store(&mut self, _greeting: GreetingCmdDto) -> Result<(), DbError>;
}

impl Debug for GreetingCommandRepositoryImpl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "GreetingRepository")
    }
}
impl GreetingCommandRepositoryImpl {
    pub async fn new(pool: Box<Pool<sqlx::Postgres>>) -> Result<Self, DbError> {
        Ok(Self { pool })
    }
}
#[async_trait]
impl GreetingCommandRepository for GreetingCommandRepositoryImpl {
    async fn store(&mut self, greeting: GreetingCmdDto) -> Result<(), DbError> {
        let mut transaction = self.pool.begin().await?;

        let id: (i64,) = sqlx::query_as("INSERT INTO greeting(message_id, \"from\", \"to\", heading, message, created) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id")
            .bind(Uuid::from_str(&*greeting.id).unwrap())
            .bind(greeting.from)
            .bind(greeting.to)
            .bind(greeting.heading)
            .bind(greeting.message)
            .bind(greeting.created)
            .fetch_one(&mut *transaction).await?;

        sqlx::query("INSERT INTO ikke_paa_logg(greeting_id) VALUES ($1)")
            .bind(id.0)
            .execute(&mut *transaction)
            .await?;

        transaction.commit().await?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GreetingCmdDto {
    id: String,
    to: String,
    from: String,
    heading: String,
    message: String,
    created: NaiveDateTime,
}
