use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PgTraceContext {
    pub trace_id: String,
    pub parent_span_id: String,
}


impl PgTraceContext  {
    pub fn to_sql(&self) -> String {
        format!("SET LOCAL pg_tracing.trace_context = 'traceparent=''00-{}-{}-01'''", self.trace_id, self.parent_span_id)
    }
}