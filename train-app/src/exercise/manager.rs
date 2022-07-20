use uuid::Uuid;

use diesel::result::Error;
use diesel::{ExpressionMethods, RunQueryDsl};
use diesel::query_dsl::methods::{FilterDsl, LimitDsl, OrderDsl};

use super::model::{Exercise, ExerciseDB};
use crate::response::Response;
use crate::db::DBPooledConnection;
pub type Exercises = Response<Exercise>;

pub fn list_exo(total_exercises: i64, conn: &DBPooledConnection) -> Result<Exercises, Error> {
    use crate::schema::exercises::dsl::*;

    let _exercises = match exercises
        .order(created_at.desc())
        .limit(total_exercises)
        .load::<ExerciseDB>(conn)
    {
        Ok(tws) => tws,
        Err(_) => vec![],
    };

    Ok(Exercises {
        results: _exercises
            .into_iter()
            .map(|t| t.to_exercise())
            .collect::<Vec<Exercise>>(),
    })
}

pub fn find_exo(_id: Uuid, conn: &DBPooledConnection) -> Result<Exercise, Error> {
    use crate::schema::exercises::dsl::*;

    let res = exercises.filter(id.eq(_id)).load::<ExerciseDB>(conn);
    match res {
        Ok(exo_db) => match exo_db.first() {
            Some(exo_db) => Ok(exo_db.to_exercise()),
            _ => Err(Error::NotFound),
        },
        Err(err) => Err(err),
    }
}

pub fn create_exo(exercise: Exercise, conn: &DBPooledConnection) -> Result<Exercise, Error> {
    use crate::schema::exercises::dsl::*;

    let exo_db = exercise.to_db();
    let _ = diesel::insert_into(exercises).values(&exo_db).execute(conn);

    Ok(exo_db.to_exercise())
}

pub fn delete_exo(_id: Uuid, conn: &DBPooledConnection) -> Result<(), Error> {
    use crate::schema::exercises::dsl::*;

    let res = diesel::delete(exercises.filter(id.eq(_id))).execute(conn);
    match res {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
