use crate::schema::nodes;
use diesel::Queryable;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateNodeDto {
    #[validate(required, length(equal = 17))]
    #[serde(alias = "m")]
    pub mac: Option<String>,

    #[validate(length(equal = 15))]
    pub ip: Option<String>,

    #[serde(alias = "n")]
    #[validate(length(max = 256))]
    pub notes: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct UpdateNodeDto {
    #[validate(required, length(equal = 17))]
    #[serde(alias = "m")]
    pub mac: Option<String>,

    #[validate(length(equal = 15))]
    pub ip: Option<String>,

    #[serde(alias = "n")]
    #[validate(length(max = 256))]
    pub notes: Option<String>,

    #[serde(alias = "s")]
    pub status: Option<i32>,

    pub ts: Option<std::time::SystemTime>,
}

#[derive(Deserialize, Validate)]
pub struct UpdateNodeStatusDto {
    #[validate(required, length(equal = 15))]
    pub ip: Option<String>,

    #[validate(required)]
    #[serde(alias = "s")]
    pub status: Option<i32>,
}

#[derive(Insertable)]
#[table_name = "nodes"]
pub struct InsertNode {
    pub mac: String,
    pub ip: Option<String>,
    pub notes: Option<String>,
}

#[derive(Insertable)]
#[table_name = "nodes"]
pub struct UpdateNode {
    pub mac: Option<String>,
    pub ip: Option<String>,
    pub notes: Option<String>,
    pub status: Option<i32>,
}

#[derive(Insertable)]
#[table_name = "nodes"]
pub struct UpdateNodeStatus {
    pub ip: String,
    pub status: i32,
}

#[derive(Clone, Serialize, Queryable)]
pub struct Node {
    pub id: i32,
    pub mac: String,
    pub ip: Option<String>,
    pub notes: Option<String>,
    pub status: Option<i32>,
    pub ts: std::time::SystemTime,
}
