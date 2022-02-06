use diesel::prelude::*;
use log::trace;

use crate::measurements::model;

type DbError = Box<dyn std::error::Error + Send + Sync>;

// It is common when using Diesel with Actix web to import schema-related
// modules inside a function's scope (rather than the normal module's scope)
// to prevent import collisions and namespace pollution.

/// Run query using Diesel to find measurements and return it.
pub fn find_all(conn: &PgConnection) -> Result<Option<Vec<model::Measurement>>, DbError> {
    use crate::schema::measurements::dsl::*;
    let result = measurements.load::<model::Measurement>(conn).optional()?;

    Ok(result)
}

/// Run query using Diesel to find measurements by id and return it.
pub fn find_by_id(conn: &PgConnection, db_id: i64) -> Result<Option<model::Measurement>, DbError> {
    use crate::schema::measurements::dsl::*;
    trace!("query db id {}", db_id);
    let result = measurements
        .filter(id.eq(db_id))
        .first::<model::Measurement>(conn)
        .optional()?;

    Ok(result)
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn create(
    conn: &PgConnection,
    data: &model::InsertMeasurement,
) -> Result<Option<bool>, DbError> {
    use crate::schema::measurements::dsl::*;
    trace!("create temperature measurement");
    let result = diesel::insert_into(measurements)
        .values(data)
        .execute(conn)?;

    Ok(Some(result == 1))
}

/// Run query using Diesel to update an existing database row.
pub fn update(
    conn: &PgConnection,
    db_id: i64,
    data: &model::UpdateMeasurement,
) -> Result<Option<bool>, DbError> {
    use crate::schema::measurements::dsl::*;
    trace!("update db id {}", db_id);
    let result = diesel::update(measurements.filter(id.eq(db_id)))
        .set(val.eq(data.val))
        .execute(conn)?;

    Ok(Some(result == 1))
}

/// Run query using Diesel to delete an existing database row.
pub fn delete(conn: &PgConnection, db_id: i64) -> Result<Option<bool>, DbError> {
    trace!("delete db id {}", db_id);
    use crate::schema::measurements::dsl::*;
    let result = diesel::delete(measurements.filter(id.eq(db_id))).execute(conn)?;

    Ok(Some(result == 1))
}
