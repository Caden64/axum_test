use axum::Extension;

#[derive(Clone)]
pub struct HeaderMessage(pub String);

pub async fn read_middleware_custom_headers(Extension(message): Extension<HeaderMessage>) -> String{
    message.0
}