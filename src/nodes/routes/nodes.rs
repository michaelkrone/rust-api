use crate::db::pool;
use crate::nodes::actions::nodes as actions;
use crate::nodes::model::nodes as model;
use actix_web::{delete, error, get, post, put, web, Error, HttpResponse};
use actix_web_validator::Json;
use uuid::Uuid;

#[get("/nodes")]
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
        let res = HttpResponse::NotFound().body(format!("No node found"));
        Ok(res)
    }
}

#[post("/nodes")]
async fn create(
    pool: web::Data<pool::DbPool>,
    dto: Json<model::CreateNodeDto>,
) -> Result<HttpResponse, Error> {
    let dto = dto.into_inner();
    let result = web::block(move || {
        let conn = pool.get()?;
        let nid = Uuid::new_v4();
        actions::create(
            &conn,
            &model::InsertNode {
                nid,
                mac: dto.mac.unwrap(),
                name: dto.name.unwrap(),
                notes: dto.notes,
                locations_id: dto.locations_id,
                applications_ids: dto.applications_ids,
            },
        )
    })
    .await
    .map_err(error::ErrorInternalServerError)?;

    if let Some(res) = result {
        Ok(HttpResponse::Created().json(res))
    } else {
        let res = HttpResponse::UnprocessableEntity().body(format!("Could not create data"));
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
                name: dto.name,
                notes: dto.notes,
                locations_id: dto.locations_id,
                applications_ids: dto.applications_ids,
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
