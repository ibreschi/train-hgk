
use actix_web::web::{Path};
use actix_web::{HttpResponse};

use crate::constants::{APPLICATION_JSON};

#[get("/health")]
pub async fn health() -> HttpResponse {
    return HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json({})
}
