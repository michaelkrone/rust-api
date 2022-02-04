use super::actions;
use super::model;
use crate::db::pool;
use actix_web::{delete, error, get, post, put, web, Error, HttpResponse};
use actix_web_validator::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum TemperatureUnit {
    K,
    C,
    F,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateTemperatureMeasurement {
    #[validate(required, range(min = -20, max = 60))]
    #[serde(alias = "value")]
    pub v: Option<f32>,

    #[serde(alias = "unit")]
    pub u: Option<TemperatureUnit>,
}

#[get("/temperatures")]
async fn find_all(pool: web::Data<pool::DbPool>) -> Result<HttpResponse, Error> {
    let result = web::block(move || {
        let conn = pool.get()?;
        actions::find_all(&conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    if let Some(result) = result {
        Ok(HttpResponse::Ok().json(result))
    } else {
        let res = HttpResponse::NotFound().body(format!("No measurements found"));
        Ok(res)
    }
}

#[get("/temperatures/{id}")]
async fn find(pool: web::Data<pool::DbPool>, id: web::Path<i64>) -> Result<HttpResponse, Error> {
    let id = id.into_inner();
    let result = web::block(move || {
        let conn = pool.get()?;
        actions::find_by_id(&conn, id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    if let Some(result) = result {
        Ok(HttpResponse::Ok().json(result))
    } else {
        let res = HttpResponse::NotFound().body(format!("No measurement found"));
        Ok(res)
    }
}

#[post("/temperatures")]
async fn create(
    pool: web::Data<pool::DbPool>,
    dto: web::Json<model::CreateTemperature>,
) -> Result<HttpResponse, Error> {
    let dto = dto.into_inner();
    let result = web::block(move || {
        let conn = pool.get()?;
        actions::create(&conn, &dto)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    if result {
        Ok(HttpResponse::Created().finish())
    } else {
        let res = HttpResponse::InternalServerError().body(format!("Error while creating data"));
        Ok(res)
    }
}

// #[put("/temperatures/{id}")]
// async fn update(
//     pool: web::Data<pool::DbPool>,
//     id: web::Path<i32>,
//     temperature: web::Json<Temperature>,
// ) -> Result<HttpResponse, ApiError> {
//     let conn = pool.get().expect("couldn't get db connection from pool");
//     let temperature = Temperatures::update(&conn, id.into_inner(), temperature.into_inner())?;
//     Ok(HttpResponse::Ok().json(temperature))
// }

// #[delete("/temperatures/{id}")]
// async fn delete(
//     pool: web::Data<pool::DbPool>,
//     id: web::Path<i32>,
// ) -> Result<HttpResponse, ApiError> {
//     let conn = pool.get().expect("couldn't get db connection from pool");
//     let deleted_temperature = Temperatures::delete(&conn, id.into_inner())?;
//     Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_temperature })))
// }

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(create);
    // cfg.service(update);
    // cfg.service(delete);
}
