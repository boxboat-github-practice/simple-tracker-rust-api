mod api;
mod models;
mod repository;
#[cfg(test)]
mod test;

//modify imports below
use actix_web::{web::Data, App, HttpServer};
use api::inventory_api::{
    create_inventory, delete_inventory, get_all_inventory, get_inventory, update_inventory,
};
use repository::mongodb_repo::MongoRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(create_inventory)
            .service(get_inventory)
            .service(update_inventory)
            .service(delete_inventory)
            .service(get_all_inventory)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
