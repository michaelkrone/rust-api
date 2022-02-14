use crate::db::pool;
use crate::nodes::actions::nodes_status as actions;
use crate::nodes::model::nodes_status as model;
use actix_web::{delete, error, get, patch, put, web, Error, HttpResponse};
use actix_web_validator::Json;

#[get("/nodes/status")]
async fn find_all(pool: web::Data<pool::DbPool>) -> Result<HttpResponse, Error> {
    let result = web::block(move || {
        let conn = pool.get()?;
        actions::find_all(&conn, None, None)
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

#[get("/nodes/status/{id}")]
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
        let res = HttpResponse::NotFound().body(format!("No node found"));
        Ok(res)
    }
}

#[patch("/nodes/status/{nodeId}")]
async fn update_by_nid(
    pool: web::Data<pool::DbPool>,
    id: web::Path<String>,
    dto: Json<model::UpdateNodeStatusDto>,
) -> Result<HttpResponse, Error> {
    let id = id.into_inner();
    let dto = dto.into_inner();
    let result = web::block(move || {
        let conn = pool.get()?;
        actions::create_or_update_by_nid(
            &conn,
            id,
            &model::UpdateNodeStatus {
                ip: dto.ip,
                status: dto.status,
            },
        )
    })
    .await
    .map_err(error::ErrorInternalServerError)?;

    if let Some(_) = result {
        Ok(HttpResponse::NoContent().finish())
    } else {
        let res = HttpResponse::NotFound().body(format!("Node not registered"));
        Ok(res)
    }
}

#[put("/nodes/status/{id}")]
async fn update(
    pool: web::Data<pool::DbPool>,
    id: web::Path<i32>,
    dto: Json<model::UpdateNodeStatusDto>,
) -> Result<HttpResponse, Error> {
    let id = id.into_inner();
    let dto = dto.into_inner();
    let result = web::block(move || {
        let conn = pool.get()?;
        actions::update(
            &conn,
            id,
            &model::UpdateNodeStatus {
                ip: dto.ip,
                status: dto.status,
            },
        )
    })
    .await
    .map_err(error::ErrorInternalServerError)?;

    if let Some(_) = result {
        Ok(HttpResponse::NoContent().finish())
    } else {
        let res = HttpResponse::NotFound().body(format!("Node not registered"));
        Ok(res)
    }
}

#[delete("/nodes/status/{id}")]
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
    cfg.service(update);
    cfg.service(update_by_nid);
    cfg.service(delete);
}
