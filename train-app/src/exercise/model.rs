use serde::{Deserialize, Serialize};
use chrono::{DateTime, NaiveDateTime, Utc, TimeZone};
use diesel::{Insertable, Queryable};

use uuid::Uuid;
use std::str::FromStr;

use super::super::schema::exercises;


#[derive(Debug, Deserialize, Serialize)]
pub struct Exercise {
    pub id: String,
    pub created_at: DateTime<Utc>,
}

impl Exercise {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
        }
    }

    pub fn to_db(&self, tweet_id: Uuid) -> ExerciseDB {
        ExerciseDB {
            id: Uuid::from_str(self.id.as_str()).unwrap(),
            created_at: self.created_at.naive_utc()
        }
    }
}

#[table_name = "exercises"]
#[derive(Queryable, Insertable)]
pub struct ExerciseDB {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
}

impl ExerciseDB {
    pub fn to_exercise(&self) -> Exercise {
        Exercise {
            id: self.id.to_string(),
            created_at: Utc.from_utc_datetime(&self.created_at),
        }
    }
}
