use axum::extract::Query;
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct QueryData {
    message: String,
    id: i32
}

pub async fn query_params(Query(data): Query<QueryData>) -> Json<QueryData> {
    Json(data)
}