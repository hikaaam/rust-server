mod finance;

use axum::Router;

pub fn main() -> Router {
    let router = Router::new().merge(finance::main());
    router
}
