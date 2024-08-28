use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow, PartialEq)]
pub struct TestModelResponse {
    pub id: i32,
    pub place: String,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct TestModel {
    pub id: i32,
    pub place: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Deserialize, Serialize)]
pub struct TestCreate {
    pub place: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct TestResponses {
    pub status: String,
    pub results: i32,
    pub test: Vec<TestModelResponse>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct TestResponse {
    pub status: String,
    pub test: TestModelResponse,
}
