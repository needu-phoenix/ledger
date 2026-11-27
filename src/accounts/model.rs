use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Account {
    pub id: i64,
    pub name: String,
    pub currency: String,
    pub allow_negative: bool,
    pub created_at: DateTime<Utc>,
}
