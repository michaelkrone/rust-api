use crate::schema::temperatures;
// use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "temperatures"]
pub struct CreateTemperature {
    // #[validate(required, range(min = -20, max = 60))]
    #[serde(alias = "v")]
    pub value: f64,

    #[serde(alias = "u")]
    pub unit: String,
    pub node: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Temperature {
    pub id: i64,
    pub value: f64,
    pub unit: String,
    pub node: i32,
    // pub ts: String,
}
