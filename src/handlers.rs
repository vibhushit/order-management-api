use crate::db::Database;
use crate::models::{CreateOrderRequest, Order, UpdateOrderStatusRequest};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use uuid::Uuid;

#[post("/orders")]
pub async fn create_order(
    db: web::Data<Database>,
    req: web::Json<CreateOrderRequest>,
) -> impl Responder {
    let order = Order::new(
        req.customer_name.clone(),
        req.items.clone(),
        req.total_amount,
    );

    match db.create_order(order) {
        Ok(order) => HttpResponse::Created().json(order),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

#[get("/orders/{id}")]
pub async fn get_order(db: web::Data<Database>, id: web::Path<Uuid>) -> impl Responder {
    match db.get_order(&id.into_inner()) {
        Ok(order) => HttpResponse::Ok().json(order),
        Err(_) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Order not found"
        })),
    }
}

#[get("/orders")]
pub async fn list_orders(db: web::Data<Database>) -> impl Responder {
    match db.list_orders() {
        Ok(orders) => HttpResponse::Ok().json(orders),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

#[put("/orders/{id}")]
pub async fn update_order_status(
    db: web::Data<Database>,
    id: web::Path<Uuid>,
    req: web::Json<UpdateOrderStatusRequest>,
) -> impl Responder {
    match db.update_order_status(&id.into_inner(), req.status.clone()) {
        Ok(order) => HttpResponse::Ok().json(order),
        Err(_) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Order not found"
        })),
    }
}

#[delete("/orders/{id}")]
pub async fn delete_order(db: web::Data<Database>, id: web::Path<Uuid>) -> impl Responder {
    match db.delete_order(&id.into_inner()) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Order not found"
        })),
    }
}
