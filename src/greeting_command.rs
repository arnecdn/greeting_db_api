use std::collections::HashMap;
use crate::greeting_pg_trace::PgTraceContext;
use crate::DbError;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use sqlx::Pool;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use uuid::Uuid;

pub struct GreetingCommandRepositoryImpl {
    pool: Box<Pool<sqlx::Postgres>>,
}

#[async_trait]
pub trait GreetingCommandRepository {
    async fn store(
        &mut self,
        trace: PgTraceContext,
        _greeting: GreetingCmdEntity,
    ) -> Result<(), DbError>;
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
    async fn store(
        &mut self,
        trace: PgTraceContext,
        greeting: GreetingCmdEntity,
    ) -> Result<(), DbError> {
        let mut transaction = self.pool.begin().await?;

        sqlx::query(&trace.to_sql())
            .execute(&mut *transaction)
            .await?;

        let id: (i64,) = sqlx::query_as(
            "
            INSERT INTO greeting(message_id,external_reference, message)
            VALUES ($1, $2, $3) RETURNING id
            ",
        )
        .bind(Uuid::from_str(&*greeting.message_id).unwrap())
        .bind(greeting.external_reference.to_string())
        .bind(Json(greeting.clone()))
        .fetch_one(&mut *transaction)
        .await?;

        sqlx::query(
            "
                INSERT INTO ikke_paa_logg(greeting_id) VALUES ($1)",
        )
        .bind(id.0)
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GreetingCmdEntity {
    external_reference: String,
    message_id: String,
    to: String,
    from: String,
    heading: String,
    message: String,
    created: NaiveDateTime,
    events_created: HashMap<String, NaiveDateTime>,
}

