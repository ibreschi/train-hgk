use crate::constants::{APPLICATION_JSON, CONNECTION_POOL_ERROR};
use crate::db::DBPool;

use actix_web::web::{Data, Json};
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
pub async fn list(pool: Data<DBPool>) -> HttpResponse {
    let conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let mut exos = web::block(move || list_exo(50, &conn)).await.unwrap();

    // let tweets_with_likes = Tweets {
    //     results: tweets
    //         .results
    //         .iter_mut()
    //         .map(|t| {
    //             let _likes = list_likes(Uuid::from_str(t.id.as_str()).unwrap(), &conn).unwrap();
    //             t.add_likes(_likes.results)
    //         })
    //         .collect::<Vec<Tweet>>(),
    // };

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(exos)
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
pub async fn get(web::Path((path,)): web::Path<(String,)>, pool: Data<DBPool>) -> HttpResponse {
    let conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let exercise =
        web::block(move || find_exo(Uuid::from_str(path.as_str()).unwrap(), &conn)).await;

    match exercise {
        Ok(exercise) => {
            // let conn = pool.get().expect(CONNECTION_POOL_ERROR);
            // let _likes = list_likes(Uuid::from_str(exercise.id.as_str()).unwrap(), &conn).unwrap();

            HttpResponse::Ok()
                .content_type(APPLICATION_JSON)
                // .json(exercise.add_likes(_likes.results))
                .json(exercise)
        }
        _ => HttpResponse::NoContent()
            .content_type(APPLICATION_JSON)
            .await
            .unwrap(),
    }
}

/// delete a exercise by its id `/exercises/{id}`
#[delete("/exercises/{id}")]
pub async fn delete(web::Path((path,)): web::Path<(String,)>, pool: Data<DBPool>) -> HttpResponse {
    // in any case return status 204
    let conn = pool.get().expect(CONNECTION_POOL_ERROR);

    let _ = web::block(move || delete_exo(Uuid::from_str(path.as_str()).unwrap(), &conn)).await;

    HttpResponse::NoContent()
        .content_type(APPLICATION_JSON)
        .await
        .unwrap()
}
