use crate::db::error::DbError;
use diesel::prelude::*;
use log::trace;

use crate::nodes::model::nodes::{self, Nodes};

// It is common when using Diesel with Actix web to import schema-related
// modules inside a function's scope (rather than the normal module's scope)
// to prevent import collisions and namespace pollution.

/// Run query using Diesel to find nodes and return it.
pub fn find_all(
    conn: &PgConnection,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Option<Vec<Nodes>>, DbError> {
    use crate::schema::nodes::dsl::*;
    let result = nodes
        .limit(limit.unwrap_or(100))
        .offset(offset.unwrap_or(0))
        .load::<Nodes>(conn)
        .optional()?;

    Ok(result)
}

/// Run query using Diesel to find nodes by id and return it.
pub fn find_by_id(conn: &PgConnection, db_id: i32) -> Result<Option<Nodes>, DbError> {
    use crate::schema::nodes::dsl::*;
    trace!("query db id {}", db_id);
    let result = nodes.filter(id.eq(db_id)).first::<Nodes>(conn).optional()?;

    Ok(result)
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn create(conn: &PgConnection, data: &nodes::InsertNode) -> Result<Option<Nodes>, DbError> {
    use crate::schema::nodes::dsl::*;
    trace!("create new node");
    let result = diesel::insert_into(nodes)
        .values(data)
        .get_result::<Nodes>(conn)?;

    Ok(Some(result))
}

/// Run query using Diesel to update an existing database row.
pub fn update(
    conn: &PgConnection,
    db_id: i32,
    data: &nodes::UpdateNode,
) -> Result<Option<bool>, DbError> {
    use crate::schema::nodes::dsl::*;
    trace!("update db id {}", db_id);
    let result = diesel::update(nodes.filter(id.eq(db_id)))
        .set((
            mac.eq(data.mac.as_ref().unwrap()),
            notes.eq(data.notes.as_ref().unwrap_or(&String::from(""))),
        ))
        .execute(conn)?;

    Ok(Some(result == 1))
}

/// Run query using Diesel to delete an existing database row.
pub fn delete(conn: &PgConnection, db_id: i32) -> Result<Option<bool>, DbError> {
    trace!("delete db id {}", db_id);
    use crate::schema::nodes::dsl::*;
    let result = diesel::delete(nodes.filter(id.eq(db_id))).execute(conn)?;

    Ok(Some(result == 1))
}
