use crate::greeting_pg_trace::PgTraceContext;
use crate::DbError;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Executor, Pool, Postgres, QueryBuilder, Row};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use sqlx::types::JsonValue;
use uuid::Uuid;

pub struct GreetingQueryRepositoryImpl {
    pool: Box<Pool<sqlx::Postgres>>,
}

#[async_trait]
pub trait GreetingQueryRepository {
    async fn list_log_entries(
        &self,
        trace: PgTraceContext,
        logg_query: LoggQueryEntity,
    ) -> Result<Vec<LoggEntryEntity>, DbError>;
    async fn last_log_entry(&self, trace: PgTraceContext) -> Result<Option<LoggEntryEntity>, DbError>;
    async fn find_greeting(
        &self,
        trace: PgTraceContext,
        greeting_id: i64
    ) -> Result<Option<GreetingEntity>, DbError>;
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
    async fn list_log_entries(
        &self,
        trace: PgTraceContext,
        logg_query: LoggQueryEntity,
    ) -> Result<Vec<LoggEntryEntity>, DbError> {
        let direction = SqlDirection::value_of(&logg_query.direction);

        let mut logg_sql: QueryBuilder<Postgres> = QueryBuilder::new(
            "SELECT l.id, \
                l.greeting_id, \
                g.message_id,
                l.created \
            FROM LOGG l \
            JOIN GREETING g ON l.greeting_id = g.id \
            ",
        );
        logg_sql.push(format!(" WHERE l.id {} ", direction.operator));
        logg_sql.push_bind(logg_query.offset);
        logg_sql.push(format!(" ORDER BY l.id {}", direction.order));
        logg_sql.push(" LIMIT ");
        logg_sql.push_bind(logg_query.limit);
        let mut transaction = self.pool.begin().await?;

        sqlx::query(&trace.to_sql())
            .execute(&mut *transaction)
            .await?;

        let r = transaction.fetch_all(logg_sql.build()).await.map(|res| {
            res.iter()
                .map(|v| LoggEntryEntity {
                    id: v.get(0),
                    greeting_id: v.get(1),
                    message_id: v.get(2),
                    created: v.get(3),
                })
                .collect::<Vec<_>>()
        })?;
        transaction.commit().await?;

        Ok(r)
    }

    async fn last_log_entry(
        &self,
        trace: PgTraceContext,
    ) -> Result<Option<LoggEntryEntity>, DbError> {
        let mut logg_sql: QueryBuilder<Postgres> = QueryBuilder::new(
            "SELECT l.id,
                   l.greeting_id,
                   g.message_id,
                   l.created
                FROM logg l
                JOIN greeting g ON l.greeting_id = g.id
                ORDER BY l.id DESC
                LIMIT 1
            ",
        );
        let mut transaction = self.pool.begin().await?;
        sqlx::query(&trace.to_sql())
            .execute(&mut *transaction)
            .await?;

        let optional_row = transaction.fetch_optional(logg_sql.build()).await?;

        let optional_log_entry = match optional_row {
            Some(r) => Some(LoggEntryEntity {
                id: r.get(0),
                greeting_id: r.get(1),
                message_id: r.get(2),
                created: r.get(3),
            }),
            None => None,
        };

        transaction.commit().await?;

        Ok(optional_log_entry)
    }

    async fn find_greeting(
        &self,
        trace: PgTraceContext,
        greeting_id: i64
    ) -> Result<Option<GreetingEntity>, DbError> {
        let  logg_sql = "SELECT g.id,
                        g.message,
                        g.created
                FROM greeting g
                where g.id = $1
            ";
        let mut transaction = self.pool.begin().await?;
        sqlx::query(&trace.to_sql())
            .execute(&mut *transaction)
            .await?;

        let optional_row = sqlx::query(&logg_sql)
            .bind(greeting_id)
            .fetch_optional(&mut *transaction)
            .await?;


        let optional_log_entry = match optional_row {
            Some(r) => Some(GreetingEntity {
                id: r.get(0),
                message: r.get(1),
                created: r.get(2),
            }),
            None => None,
        };

        transaction.commit().await?;

        Ok(optional_log_entry)
    }
}

struct SqlDirection {
    order: String,
    operator: String,
}

impl SqlDirection {
    fn value_of(direction: &str) -> SqlDirection {
        match direction {
            "forward" => SqlDirection {
                order: String::from("ASC"),
                operator: String::from(">="),
            },
            "backward" => SqlDirection {
                order: String::from("DESC"),
                operator: String::from("<="),
            },
            _ => panic!("Invalid direction"),
        }
    }
}

pub struct LoggQueryEntity {
    pub offset: i64,
    pub limit: i64,
    pub direction: String,
}
pub struct LoggEntryEntity {
    pub id: i64,
    pub greeting_id: i64,
    pub message_id: Uuid,
    pub created: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GreetingEntity{
    pub id: i64,
    pub message: JsonValue,
    pub created: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GreetingMessageEntity {
    external_reference: String,
    message_id: String,
    to: String,
    from: String,
    heading: String,
    message: String,
    pub created: DateTime<Utc>,
    events_created: HashMap<String, DateTime<Utc>>,
}
