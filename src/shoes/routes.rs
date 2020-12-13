use actix_web::{delete, get, post, put, web};
use actix_web::web::Json;
use sqlx::SqlitePool;

use crate::{common};
use crate::errors::ApiError;
use crate::shoes::{Shoe, ShoeUsedView, UpdateShoe};
use crate::middleware::Auth;
use crate::common::{make_api_response, DynamicResult};

#[derive(Serialize)]
pub struct ListShoesResult {
    pub result: String,
    pub count: i32,
    pub shoes: Vec<ShoeUsedView>,
}

#[get("")]
async fn list_all(db_pool: web::Data<SqlitePool>) -> Result<Json<ListShoesResult>, ApiError> {
    let shoes = Shoe::find_all(db_pool.get_ref()).await?;
    Ok(Json(ListShoesResult {
        result: common::SUCCESS_MSG.to_string(),
        count: shoes.len() as i32,
        shoes,
    }))
}


#[post("")]
async fn create(shoe: web::Json<Shoe>, db_pool: web::Data<SqlitePool>) -> DynamicResult {
    let new_id = Shoe::create(shoe.into_inner(), db_pool.get_ref()).await?;
    Ok(Json(make_api_response(true, vec![("id".to_string(), new_id.to_string())])))
}

#[put("/{id}")]
async fn update(id: web::Path<i32>, shoe_data: web::Json<UpdateShoe>, db_pool: web::Data<SqlitePool>) -> Result<Json<Shoe>, ApiError> {
    let result = Shoe::update(id.into_inner(), shoe_data.into_inner(), db_pool.get_ref()).await?;
    return Ok(Json(result));
}

#[delete("/{id}")]
async fn delete(id: web::Path<i32>, db_pool: web::Data<SqlitePool>) -> DynamicResult {
    Shoe::delete(id.into_inner(), db_pool.get_ref()).await?;
    return Ok(Json(make_api_response(true, vec![])));
}


// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/shoes")
            .wrap(Auth)
            .service(list_all)
            // .service(details)
            .service(create)
            .service(update)
            .service(delete)
    );
}