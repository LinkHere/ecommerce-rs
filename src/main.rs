use axum;
use dotenv::dotenv;
//use backend::{db::establish_connection, routes::create_router};
mod util;
mod services;
#[tokio::main]
async fn main() {
    dotenv().ok();
    let db = util::db::establish_connection().await;
    let app = util::routes::create_router(db);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
