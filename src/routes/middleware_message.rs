use axum::Extension;
use crate::routes::SharedData;

pub async fn middleware_message(Extension(shared): Extension<SharedData>)  -> String{
    shared.message
}