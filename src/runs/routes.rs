

use actix_web::{delete, get, post, put, web};
use actix_web::web::Json;
use sqlx::SqlitePool;

use crate::{common};
use crate::common::{DynamicResult, make_api_response};
use crate::errors::ApiError;
use crate::middleware::Auth;
use crate::runs::{Run, UpdateRun};

#[derive(Serialize)]
pub struct ListRunsResult {
    pub result: String,
    pub count: i32,
    pub runs: Vec<Run>,
}


#[get("")]
async fn list_all(db_pool: web::Data<SqlitePool>) -> Result<Json<ListRunsResult>, ApiError> {
    let runs = Run::find_all(db_pool.get_ref()).await?;
    Ok(Json(ListRunsResult {
        result: common::SUCCESS_MSG.to_string(),
        count: runs.len() as i32,
        runs,
    }))
}

#[get("/{id}")]
async fn details(id: web::Path<i32>, db_pool: web::Data<SqlitePool>) -> Result<Json<Run>, ApiError> {
    let run_opt = Run::find_by_id(id.into_inner(), db_pool.get_ref()).await?;
    match run_opt {
        Some(run) => Ok(Json(run)),
        None => Err(ApiError::NotFound("Run not found".to_string()))
    }
}

#[post("")]
async fn create(run: web::Json<Run>, db_pool: web::Data<SqlitePool>) -> DynamicResult {
    let new_id = Run::create(run.into_inner(), db_pool.get_ref()).await?;
    Ok(Json(make_api_response(true, vec![("id".to_string(), new_id.to_string())])))
}

#[put("/{id}")]
async fn update(id: web::Path<i32>, run_data: web::Json<UpdateRun>, db_pool: web::Data<SqlitePool>) -> Result<Json<Run>, ApiError> {
    let result = Run::update(id.into_inner(), run_data.into_inner(), db_pool.get_ref()).await?;
    return Ok(Json(result));
}

#[delete("/{id}")]
async fn delete(id: web::Path<i32>, db_pool: web::Data<SqlitePool>) -> DynamicResult {
    Run::delete(id.into_inner(), db_pool.get_ref()).await?;
    return Ok(Json(make_api_response(true, vec![])));
}


// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/runs")
            .wrap(Auth)
            .service(list_all)
            .service(details)
            .service(create)
            .service(update)
            .service(delete)
    );
}