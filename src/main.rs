use actix_web::{App, HttpServer};
use order_management_api::{db::Database, handlers};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::new();

    println!("Server starting at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(db.clone()))
            .service(handlers::create_order)
            .service(handlers::get_order)
            .service(handlers::list_orders)
            .service(handlers::update_order_status)
            .service(handlers::delete_order)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
