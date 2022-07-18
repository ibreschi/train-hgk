use crate::constants::{APPLICATION_JSON};
use actix_web::{HttpResponse};
use actix_web::web::{Path};

use super::model::Exercise;


#[get("/exercises/")]
pub async fn list(path: Path<(String,)>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(Exercise::new())
}
