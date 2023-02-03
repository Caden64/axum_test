use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Data {
    message: String,
    count: i32,
    username: String
}

pub async fn get_json() -> Json<Data> {
    let data = Data {
        message: "Hello, from data".to_string(),
        count:  3,
        username: "winnietthepooh".to_string()
    };
    Json(data)
}