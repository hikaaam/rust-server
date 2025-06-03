use axum::routing::post;
use axum::{Router, routing::get};

mod add_finance;
mod get_finance;
mod get_finance_detail;

pub fn main() -> Router {
    let router = Router::new()
        .route("/v1/finance", get(get_finance::main))
        .route("/v1/finance/{id}", get(get_finance_detail::main))
        .route("/v1/finance", post(add_finance::main));
    router
}
