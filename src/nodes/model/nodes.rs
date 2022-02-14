use crate::schema::nodes;
use diesel::Queryable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateNodeDto {
    #[validate(required, length(equal = 17))]
    pub mac: Option<String>,

    #[validate(required, length(max = 128))]
    pub name: Option<String>,

    #[validate(length(max = 256))]
    pub notes: Option<String>,

    #[serde(alias = "locationsId")]
    pub locations_id: Option<i32>,

    #[serde(alias = "applicationIds")]
    pub applications_ids: Option<Vec<i32>>,
}

#[derive(Deserialize, Validate)]
pub struct UpdateNodeDto {
    #[validate(length(equal = 17))]
    pub mac: Option<String>,

    #[validate(length(max = 128))]
    pub name: Option<String>,

    #[validate(length(max = 256))]
    pub notes: Option<String>,

    #[serde(alias = "locationsId")]
    pub locations_id: Option<i32>,

    #[serde(alias = "applicationIds")]
    pub applications_ids: Option<Vec<i32>>,
}

#[derive(Insertable)]
#[table_name = "nodes"]
pub struct InsertNode {
    pub nid: Uuid,
    pub mac: String,
    pub name: String,
    pub notes: Option<String>,
    pub locations_id: Option<i32>,
    pub applications_ids: Option<Vec<i32>>,
}

#[derive(Insertable)]
#[table_name = "nodes"]
pub struct UpdateNode {
    pub mac: Option<String>,
    pub name: Option<String>,
    pub notes: Option<String>,
    pub locations_id: Option<i32>,
    pub applications_ids: Option<Vec<i32>>,
}

#[derive(Serialize, Queryable)]
pub struct Nodes {
    pub id: i32,
    pub nid: Uuid,
    pub mac: String,
    pub name: String,
    pub notes: Option<String>,
    pub locations_id: Option<i32>,
    pub applications_ids: Option<Vec<i32>>,
    pub ts: std::time::SystemTime,
}
