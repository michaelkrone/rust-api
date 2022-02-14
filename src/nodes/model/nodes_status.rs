use super::nodes::Nodes;
use crate::schema::nodes_status;
use diesel::Queryable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct UpdateNodeStatusDto {
    #[validate(length(min = 11, max = 15))]
    pub ip: Option<String>,

    #[validate(required)]
    #[serde(alias = "s")]
    pub status: Option<i32>,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "nodes_status"]
pub struct CreateNodeStatus {
    pub nodes_id: i32,
    pub nid: Uuid,
    pub ip: Option<String>,
    pub status: Option<i32>,
    pub ts: std::time::SystemTime,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "nodes_status"]
pub struct UpdateNodeStatus {
    pub ip: Option<String>,
    pub status: Option<i32>,
}

#[derive(Serialize, Identifiable, Queryable, Associations, PartialEq)]
#[belongs_to(Nodes)]
#[table_name = "nodes_status"]
pub struct NodesStatus {
    pub id: i32,
    pub nodes_id: i32,
    pub nid: Uuid,
    pub ip: Option<String>,
    pub status: Option<i32>,
    pub ts: std::time::SystemTime,
}
