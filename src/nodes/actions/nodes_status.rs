use std::time::SystemTime;

use crate::db::error::DbError;
use diesel::prelude::*;
use log::trace;
use uuid::Uuid;

use crate::nodes::model::nodes_status::{CreateNodeStatus, NodesStatus, UpdateNodeStatus};

// It is common when using Diesel with Actix web to import schema-related
// modules inside a function's scope (rather than the normal module's scope)
// to prevent import collisions and namespace pollution.

/// Run query using Diesel to find nodes and return it.
pub fn find_all(
    conn: &PgConnection,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Option<Vec<NodesStatus>>, DbError> {
    use crate::schema::nodes_status::dsl::*;
    let result = nodes_status
        .limit(limit.unwrap_or(100))
        .offset(offset.unwrap_or(0))
        .load::<NodesStatus>(conn)
        .optional()?;

    Ok(result)
}

/// Run query using Diesel to find nodes by id and return it.
pub fn find_by_id(conn: &PgConnection, db_id: i32) -> Result<Option<NodesStatus>, DbError> {
    use crate::schema::nodes_status::dsl::*;
    trace!("query db id {}", db_id);
    let result = nodes_status
        .filter(id.eq(db_id))
        .first::<NodesStatus>(conn)
        .optional()?;

    Ok(result)
}

/// Run query using Diesel to find nodes by id and return it.
pub fn find_by_nid(conn: &PgConnection, db_id: Uuid) -> Result<Option<NodesStatus>, DbError> {
    use crate::schema::nodes_status::dsl::*;
    trace!("query db nid {}", db_id);
    let result = nodes_status
        .filter(nid.eq(db_id))
        .first::<NodesStatus>(conn)
        .optional()?;

    Ok(result)
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn create(
    conn: &PgConnection,
    data: &CreateNodeStatus,
) -> Result<Option<NodesStatus>, DbError> {
    use crate::schema::nodes_status::dsl::*;
    trace!("create new node status");
    let result = diesel::insert_into(nodes_status)
        .values(data)
        .get_result::<NodesStatus>(conn)?;

    Ok(Some(result))
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn create_or_update_by_nid(
    conn: &PgConnection,
    db_nid: Uuid,
    data: &UpdateNodeStatus,
) -> Result<Option<bool>, DbError> {
    trace!("create or update node status");
    let by_nid = find_by_nid(conn, db_nid)?;
    if let Some(res) = by_nid {
        return update(conn, res.id, data);
    } else {
        let result = create(
            conn,
            &CreateNodeStatus {
                nodes_id: 1,
                nid: db_nid,
                ip: data.ip,
                status: data.status,
                ts: SystemTime::now(),
            },
        )?;
        match result {
            Some(_) => Ok(Some(true)),
            None => Ok(Some(false)),
        }
    }
}

/// Run query using Diesel to update an existing database row.
pub fn update(
    conn: &PgConnection,
    db_id: i32,
    data: &UpdateNodeStatus,
) -> Result<Option<bool>, DbError> {
    use crate::schema::nodes_status::dsl::*;
    trace!("update db id {}", db_id);
    let result = diesel::update(nodes_status.filter(id.eq(db_id)))
        .set((
            ip.eq(&data.ip),
            status.eq(data.status),
            ts.eq(SystemTime::now()),
        ))
        .execute(conn)?;

    Ok(Some(result == 1))
}

/// Run query using Diesel to delete an existing database row.
pub fn delete(conn: &PgConnection, db_id: i32) -> Result<Option<bool>, DbError> {
    trace!("delete db id {}", db_id);
    use crate::schema::nodes_status::dsl::*;
    let result = diesel::delete(nodes_status.filter(id.eq(db_id))).execute(conn)?;

    Ok(Some(result == 1))
}
