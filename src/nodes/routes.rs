use crate::db::pool;
use crate::nodes::actions;
use crate::nodes::model;
use actix_web::{delete, error, get, post, put, web, Error, HttpResponse};
use actix_web_validator::Json;
use actix_web_validator::Validate;

#[get("/nodes")]
async fn find_all(pool: web::Data<pool::DbPool>) -> Result<HttpResponse, Error> {
    let result = web::block(move || {
        let conn = pool.get()?;
        actions::find_all(&conn)
    })
    .await
    .map_err(error::ErrorInternalServerError)?;

    if let Some(result) = result {
        Ok(HttpResponse::Ok().json(result))
    } else {
        let res = HttpResponse::NotFound().body(format!("No nodes found"));
        Ok(res)
    }
}

#[get("/nodes/{id}")]
async fn find(pool: web::Data<pool::DbPool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let id = id.into_inner();
    let result = web::block(move || {
        let conn = pool.get()?;
        actions::find_by_id(&conn, id)
    })
    .await
    .map_err(error::ErrorInternalServerError)?;

    if let Some(result) = result {
        Ok(HttpResponse::Ok().json(result))
    } else {
        let res = HttpResponse::NotFound().body(format!("No measurement found"));
        Ok(res)
    }
}

#[post("/nodes")]
async fn create(
    pool: web::Data<pool::DbPool>,
    dto: Json<model::CreateNodeDto>,
) -> Result<HttpResponse, Error> {
    match dto.validate() {
        Err(e) => return Ok(HttpResponse::BadRequest().json(e)),
        Ok(data) => data,
    }

    let dto = dto.into_inner();
    let result = web::block(move || {
        let conn = pool.get()?;
        actions::create(
            &conn,
            &model::InsertNode {
                mac: dto.mac.unwrap(),
                ip: dto.ip,
                notes: dto.notes,
            },
        )
    })
    .await
    .map_err(error::ErrorInternalServerError)?;

    if let Some(_) = result {
        Ok(HttpResponse::Created().finish())
    } else {
        let res = HttpResponse::InternalServerError().body(format!("Error while creating data"));
        Ok(res)
    }
}

#[put("/nodes/{mac}/status")]
async fn status(
    pool: web::Data<pool::DbPool>,
    mac: web::Path<String>,
    dto: Json<model::UpdateNodeStatusDto>,
) -> Result<HttpResponse, Error> {
    match dto.validate() {
        Err(e) => return Ok(HttpResponse::BadRequest().json(e)),
        Ok(data) => data,
    }

    let mac = mac.into_inner();
    let dto = dto.into_inner();
    let result = web::block(move || {
        let conn = pool.get()?;
        actions::update_status_by_mac(
            &conn,
            mac,
            &model::UpdateNodeStatus {
                ip: dto.ip.unwrap(),
                status: dto.status.unwrap(),
            },
        )
    })
    .await
    .map_err(error::ErrorInternalServerError)?;

    if let Some(_) = result {
        Ok(HttpResponse::Created().finish())
    } else {
        let res = HttpResponse::NotFound().body(format!("Node not registered"));
        Ok(res)
    }
}

#[put("/nodes/{id}")]
async fn update(
    pool: web::Data<pool::DbPool>,
    id: web::Path<i32>,
    dto: Json<model::UpdateNodeDto>,
) -> Result<HttpResponse, Error> {
    let id = id.into_inner();
    let dto = dto.into_inner();
    let result = web::block(move || {
        let conn = pool.get()?;
        actions::update(
            &conn,
            id,
            &model::UpdateNode {
                mac: dto.mac,
                ip: dto.ip,
                notes: dto.notes,
                status: dto.status,
            },
        )
    })
    .await
    .map_err(error::ErrorInternalServerError)?;

    if let Some(_) = result {
        Ok(HttpResponse::NoContent().finish())
    } else {
        let res = HttpResponse::InternalServerError().body(format!("Error while updating data"));
        Ok(res)
    }
}

#[delete("/nodes/{id}")]
async fn delete(pool: web::Data<pool::DbPool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let id = id.into_inner();
    let result = web::block(move || {
        let conn = pool.get()?;
        actions::delete(&conn, id)
    })
    .await
    .map_err(error::ErrorInternalServerError)?;

    if let Some(_) = result {
        Ok(HttpResponse::NoContent().finish())
    } else {
        let res = HttpResponse::InternalServerError().body(format!("Error while creating data"));
        Ok(res)
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}
