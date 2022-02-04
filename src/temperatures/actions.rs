use diesel::prelude::*;
use log::trace;

use super::model;

type DbError = Box<dyn std::error::Error + Send + Sync>;

// It is common when using Diesel with Actix web to import schema-related
// modules inside a function's scope (rather than the normal module's scope)
// to prevent import collisions and namespace pollution.

/// Run query using Diesel to find measurements and return it.
pub fn find_all(conn: &PgConnection) -> Result<Option<Vec<model::Temperature>>, DbError> {
    use crate::schema::temperatures::dsl::*;
    let result = temperatures.load::<model::Temperature>(conn).optional()?;

    Ok(result)
}

/// Run query using Diesel to find measurements by id and return it.
pub fn find_by_id(conn: &PgConnection, db_id: i64) -> Result<Option<model::Temperature>, DbError> {
    use crate::schema::temperatures::dsl::*;
    trace!("query db id {}", db_id);
    let result = temperatures
        .filter(id.eq(db_id))
        .first::<model::Temperature>(conn)
        .optional()?;

    Ok(result)
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn create(conn: &PgConnection, data: &model::CreateTemperature) -> Result<bool, DbError> {
    use crate::schema::temperatures::dsl::*;
    diesel::insert_into(temperatures)
        .values(data)
        .execute(conn)?;

    Ok(true)
}
