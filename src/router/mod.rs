mod v1;
use axum::Router;
use serde::Serialize;

#[derive(Serialize)]
pub struct Message {
    message: String,
}

pub fn main() -> Router {
    let router = Router::new()
    .merge(v1::main());
    router
}

