use axum::{Json, Router};
use axum::routing::get;
use serde_json::{json, Value};

async fn get_contract() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

async fn add_contract() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

async fn delete_contract() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

pub fn get_routes() -> Router {
    Router::new().route("/contract", get(get_contract).post(add_contract).delete(delete_contract))
}
