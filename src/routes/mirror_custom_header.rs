use axum::http::HeaderMap;

pub async fn mirror_custom_header(headers: HeaderMap) -> String {
    let message = headers.get("x-message").unwrap();
    let message = message.to_str().unwrap().to_string();
    message
}