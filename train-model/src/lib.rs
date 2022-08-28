
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HelloResponse {
    pub name: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExerciseRequest {
    pub message: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response<T> {
    pub results: Vec<T>,
}

impl<T> Response<T> {
    pub fn new() -> Self {
        Self { results: vec![] }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Exercise {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub message: String,
}
