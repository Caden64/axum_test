mod hello_world;
mod mirror_body_string;
mod mirror_body_json;
mod path_variables;
mod query_params;
mod mirror_user_agent;
mod mirror_custom_header;
mod middleware_message;
mod read_middleware_custom_headers;
mod set_middleware_custom_headers;

use axum::{Extension, middleware, Router, routing::get, routing::post};
use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};
use hello_world::hello_world;
use mirror_body_string::mirror_body_string;
use mirror_body_json::mirror_body_json;
use path_variables::{path_variables, hardcoded_path};
use query_params::query_params;
use mirror_user_agent::mirror_user_agent;
use mirror_custom_header::mirror_custom_header;
use middleware_message::middleware_message;
use read_middleware_custom_headers::read_middleware_custom_headers;
use set_middleware_custom_headers::set_middleware_custom_headers;

#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

pub fn create_router() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let shared_data = SharedData{message: "Hello, from shared message".to_string()};

    Router::new()
        .route("/read_middleware_custom_header", get(read_middleware_custom_headers))
        .route_layer(middleware::from_fn(set_middleware_custom_headers))
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variables/:id", get(path_variables))
        .route("/path_variables/15", get(hardcoded_path))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
        .route("/middleware_message", get(middleware_message))
        .layer(cors)
        .layer(Extension(shared_data))

}
