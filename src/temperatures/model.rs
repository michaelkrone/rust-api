use serde::{Deserialize, Serialize};

use crate::schema::temperatures;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "temperatures"]
pub struct CreateTemperature {
    pub value: f64,
    pub unit: String,
    pub node: i32,
    // pub ts: Option<chrono::DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Temperature {
    pub id: i64,
    pub value: f64,
    pub unit: String,
    pub node: i32,
    // pub ts: Option<chrono::DateTime<Utc>>,
}
