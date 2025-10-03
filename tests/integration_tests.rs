use actix_web::{App, test, web};
use order_management_api::{db::Database, handlers, models::OrderStatus};
use serde_json::json;

#[actix_web::test]
async fn test_create_order() {
    let db = Database::new();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(handlers::create_order),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/orders")
        .set_json(json!({
            "customer_name": "Vibhushit",
            "items": "Laptop, Mouse",
            "total_amount": 85000.00
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_get_order() {
    let db = Database::new();
    let order =
        order_management_api::models::Order::new("John".to_string(), "Phone".to_string(), 60000.00);
    let order_id = order.id;
    db.create_order(order).unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(handlers::get_order),
    )
    .await;

    let req = test::TestRequest::get()
        .uri(&format!("/orders/{}", order_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_get_nonexistent_order() {
    let db = Database::new();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(handlers::get_order),
    )
    .await;

    let fake_id = uuid::Uuid::new_v4();
    let req = test::TestRequest::get()
        .uri(&format!("/orders/{}", fake_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
async fn test_list_orders() {
    let db = Database::new();
    let order1 = order_management_api::models::Order::new(
        "Customer 1".to_string(),
        "Item A".to_string(),
        100.00,
    );
    let order2 = order_management_api::models::Order::new(
        "Customer 2".to_string(),
        "Item B".to_string(),
        200.00,
    );
    db.create_order(order1).unwrap();
    db.create_order(order2).unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(handlers::list_orders),
    )
    .await;

    let req = test::TestRequest::get().uri("/orders").to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_update_order_status() {
    let db = Database::new();
    let order = order_management_api::models::Order::new(
        "John".to_string(),
        "Tablet".to_string(),
        60000.00,
    );
    let order_id = order.id;
    db.create_order(order).unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(handlers::update_order_status),
    )
    .await;

    let req = test::TestRequest::put()
        .uri(&format!("/orders/{}", order_id))
        .set_json(json!({
            "status": "completed"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let updated_order = db.get_order(&order_id).unwrap();
    assert_eq!(updated_order.status, OrderStatus::Completed);
}

#[actix_web::test]
async fn test_delete_order() {
    let db = Database::new();
    let order = order_management_api::models::Order::new(
        "John".to_string(),
        "Monitor".to_string(),
        5000.00,
    );
    let order_id = order.id;
    db.create_order(order).unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(handlers::delete_order),
    )
    .await;

    let req = test::TestRequest::delete()
        .uri(&format!("/orders/{}", order_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 204);

    assert!(db.get_order(&order_id).is_err());
}

#[actix_web::test]
async fn test_delete_nonexistent_order() {
    let db = Database::new();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(handlers::delete_order),
    )
    .await;

    let fake_id = uuid::Uuid::new_v4();
    let req = test::TestRequest::delete()
        .uri(&format!("/orders/{}", fake_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}
