use crate::schema::measurements;
use diesel::Queryable;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateMeasurementDto {
    #[validate(required, range(min = -25, max = 80))]
    #[serde(alias = "v")]
    pub val: Option<f64>,

    #[serde(alias = "n")]
    #[validate(required)]
    pub node: Option<String>,

    #[serde(alias = "t")]
    #[validate(length(min = 1, max = 4))]
    pub typ: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct UpdateMeasurementDto {
    #[validate(required, range(min = -25, max = 50))]
    #[serde(alias = "v")]
    pub val: Option<f64>,
}

#[derive(Insertable)]
#[table_name = "measurements"]
pub struct InsertMeasurement {
    pub val: f64,
    pub typ: String,
    pub node: String,
}

#[derive(Insertable)]
#[table_name = "measurements"]
pub struct UpdateMeasurement {
    pub val: f64,
}

#[derive(Clone, Serialize, Queryable)]
pub struct Measurement {
    pub id: i64,
    pub val: f64,
    pub typ: String,
    pub node: String,
    pub ts: std::time::SystemTime,
}
