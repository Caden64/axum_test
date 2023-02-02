use crate::routes::SharedData;
use axum::Extension;

pub async fn middleware_message(Extension(shared): Extension<SharedData>) -> String {
    shared.message
}
