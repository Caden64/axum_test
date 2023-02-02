use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MirrorJson {
    message: String,
}

pub async fn mirror_body_json(Json(body): Json<MirrorJson>) -> String {
    body.message
}

