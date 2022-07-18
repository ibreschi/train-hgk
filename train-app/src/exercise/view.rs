use crate::constants::{APPLICATION_JSON, CONNECTION_POOL_ERROR};

use actix_web::web::{Data, Json, Path};
use actix_web::{web, HttpResponse};

use super::model::Exercise;
use super::manager::{list_exo, create_exo, find_exo, delete_exo};

use serde::{Deserialize, Serialize};

use uuid::Uuid;
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize)]
pub struct ExerciseRequest {
    pub message: Option<String>,
}

impl ExerciseRequest {
    pub fn to_exercise(&self) -> Option<Exercise> {
        match &self.message {
            Some(message) => Some(Exercise::new(message.to_string())),
            None => None,
        }
    }
}


#[get("/exercises")]
pub async fn list() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(Exercise::new("".to_string()))
}

#[post("/exercises")]
pub async fn create(exo_req: Json<ExerciseRequest>, pool: Data<DBPool>) -> HttpResponse {
    let conn = pool.get().expect(CONNECTION_POOL_ERROR);

    let exercise = web::block(move || create_exo(exo_req.to_exercise().unwrap(), &conn)).await;

    match exercise {
        Ok(exercise) => HttpResponse::Created()
            .content_type(APPLICATION_JSON)
            .json(exercise),
        _ => HttpResponse::NoContent().await.unwrap(),
    }
}

/// find a exercise by its id `/exercises/{id}`
#[get("/exercises/{id}")]
pub async fn get(path: Path<(String,)>, pool: Data<DBPool>) -> HttpResponse {
    let conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let my_string = path.0.0.as_str();
    let exercise =
        web::block(move || find_exo(Uuid::from_str(my_string).unwrap(), &conn)).await;

    match exercise {
        Ok(exercise) => {
            let conn = pool.get().expect(CONNECTION_POOL_ERROR);
            // let _likes = list_likes(Uuid::from_str(exercise.id.as_str()).unwrap(), &conn).unwrap();

            HttpResponse::Ok()
                .content_type(APPLICATION_JSON)
                // .json(exercise.add_likes(_likes.results))
                .json({})
        }
        _ => HttpResponse::NoContent()
            .content_type(APPLICATION_JSON)
            .await
            .unwrap(),
    }
}

/// delete a exercise by its id `/exercises/{id}`
#[delete("/exercises/{id}")]
pub async fn delete(path: Path<(String,)>, pool: Data<DBPool>) -> HttpResponse {
    // in any case return status 204
    let conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let my_string = path.0.0.as_str();

    let _ = web::block(move || delete_exo(Uuid::from_str(my_string).unwrap(), &conn)).await;

    HttpResponse::NoContent()
        .content_type(APPLICATION_JSON)
        .await
        .unwrap()
}
