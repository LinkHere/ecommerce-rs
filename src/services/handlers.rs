use axum::{Json, extract::State};
use sea_orm::{EntityTrait, QueryOrder};
use crate::{services::models::Entity as Product};
use serde::Serialize;
use crate::services::models::Column;

#[derive(Serialize)]
pub struct ProductResponse {
    id: i32,
    name: String,
    price: f64,
    stock: i32,
}

pub async fn get_products(State(db): State<sea_orm::DatabaseConnection>) -> Json<Vec<ProductResponse>> {
    let products = Product::find()
        .order_by_asc(Column::Id)
        .all(&db)
        .await
        .expect("Query failed");

    let response: Vec<ProductResponse> = products.into_iter().map(|p| ProductResponse {
        id: p.id,
        name: p.name,
        price: p.price,
        stock: p.stock,
    }).collect();

    Json(response)
}
