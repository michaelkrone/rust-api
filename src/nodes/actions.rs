use diesel::prelude::*;
use log::trace;

use crate::nodes::model;

type DbError = Box<dyn std::error::Error + Send + Sync>;

// It is common when using Diesel with Actix web to import schema-related
// modules inside a function's scope (rather than the normal module's scope)
// to prevent import collisions and namespace pollution.

/// Run query using Diesel to find nodes and return it.
pub fn find_all(conn: &PgConnection) -> Result<Option<Vec<model::Node>>, DbError> {
    use crate::schema::nodes::dsl::*;
    let result = nodes.load::<model::Node>(conn).optional()?;

    Ok(result)
}

/// Run query using Diesel to find nodes by id and return it.
pub fn find_by_id(conn: &PgConnection, db_id: i32) -> Result<Option<model::Node>, DbError> {
    use crate::schema::nodes::dsl::*;
    trace!("query db id {}", db_id);
    let result = nodes
        .filter(id.eq(db_id))
        .first::<model::Node>(conn)
        .optional()?;

    Ok(result)
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn create(conn: &PgConnection, data: &model::InsertNode) -> Result<Option<bool>, DbError> {
    use crate::schema::nodes::dsl::*;
    trace!("create new node");
    let result = diesel::insert_into(nodes).values(data).execute(conn)?;

    Ok(Some(result == 1))
}

/// Run query using Diesel to update an existing database row.
pub fn update(
    conn: &PgConnection,
    db_id: i32,
    data: &model::UpdateNode,
) -> Result<Option<bool>, DbError> {
    use crate::schema::nodes::dsl::*;
    trace!("update db id {}", db_id);
    let result = diesel::update(nodes.filter(id.eq(db_id)))
        .set((
            mac.eq(data.mac.as_ref().unwrap()),
            ip.eq(data.ip.as_ref().unwrap()),
            notes.eq(data.notes.as_ref().unwrap()),
            status.eq(data.status.as_ref().unwrap()),
        ))
        .execute(conn)?;

    Ok(Some(result == 1))
}

/// Run query using Diesel to update an existing database row.
pub fn update_status_by_mac(
    conn: &PgConnection,
    mac_address: String,
    data: &model::UpdateNodeStatus,
) -> Result<Option<model::Node>, DbError> {
    use crate::schema::nodes::dsl::*;
    trace!("update db mac {}", mac_address);
    let result = diesel::update(nodes.filter(mac.eq(mac_address)))
        .set((ip.eq(&data.ip), status.eq(data.status)))
        .get_result::<model::Node>(conn)?;

    Ok(Some(result))
}

/// Run query using Diesel to delete an existing database row.
pub fn delete(conn: &PgConnection, db_id: i32) -> Result<Option<bool>, DbError> {
    trace!("delete db id {}", db_id);
    use crate::schema::nodes::dsl::*;
    let result = diesel::delete(nodes.filter(id.eq(db_id))).execute(conn)?;

    Ok(Some(result == 1))
}
