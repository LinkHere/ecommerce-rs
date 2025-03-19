use axum::{routing::get, Router};
//use crate::handlers::get_products;
use sea_orm::DatabaseConnection;
use crate::services::handlers::get_products;

pub fn create_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/products", get(get_products))
        .with_state(db)
}
