use crate::DbError;
use async_trait::async_trait;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Executor, Pool, Postgres, QueryBuilder, Row};
use std::fmt::{Debug, Formatter};
use crate::greeting_pg_trace::PgTraceContext;

pub struct GreetingQueryRepositoryImpl {
    pool: Box<Pool<sqlx::Postgres>>,
}

#[async_trait]
pub trait GreetingQueryRepository {
    async fn list_log_entries(&self, trace: PgTraceContext, logg_query: LoggQueryEntity) -> Result<Vec<LoggEntryEntity>, DbError>;
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
    async fn list_log_entries(&self, trace: PgTraceContext, logg_query: LoggQueryEntity) -> Result<Vec<LoggEntryEntity>, DbError> {
        let direction = SqlDirection::value_of(&logg_query.direction);

        let mut logg_sql: QueryBuilder<Postgres> =
            QueryBuilder::new("SELECT id, greeting_id, opprettet FROM LOGG");
        logg_sql.push(format!(" WHERE id {} ", direction.operator));
        logg_sql.push_bind(logg_query.offset);
        logg_sql.push(format!(" ORDER BY id {}", direction.order));
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
                    created: v.get(2),
                })
                .collect::<Vec<_>>()
        })?;
        transaction.commit().await?;

        Ok(r)
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
    pub created: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GreetingQueryEntity {
    id: String,
    to: String,
    from: String,
    heading: String,
    message: String,
    created: NaiveDateTime,
}
