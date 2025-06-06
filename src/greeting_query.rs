use std::fmt::{Debug, Formatter};
use async_trait::async_trait;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{Error, Pool};
use crate::DbError;

pub struct GreetingQueryRepositoryImpl {
    pool: Box<Pool<sqlx::Postgres>>,
}

#[async_trait]
pub trait GreetingQueryRepository {
    async fn store(&mut self, greeting: GreetingQueryDto) -> Result<(), DbError>;
}

impl Debug for GreetingQueryRepositoryImpl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "GreetingRepository")
    }
}
impl GreetingQueryRepositoryImpl {
    pub async fn new(pool: Box<Pool<sqlx::Postgres>>) -> Result<Self, DbError> {
        Ok(Self { pool })
    }
}
#[async_trait]
impl GreetingQueryRepository for GreetingQueryRepositoryImpl {
    async fn store(&mut self, _greeting: GreetingQueryDto) -> Result<(), DbError> {
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GreetingQueryDto {
    id: String,
    to: String,
    from: String,
    heading: String,
    message: String,
    created: NaiveDateTime,
}
