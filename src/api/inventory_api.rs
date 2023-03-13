use crate::{models::inventory_model::Inventory, repository::mongodb_repo::MongoRepo};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};
use mongodb::bson::oid::ObjectId;

#[post("/inventory")]
pub async fn create_inventory(db: Data<MongoRepo>, new_inventory: Json<Inventory>) -> HttpResponse {
    let data = Inventory {
        id: None,
        name: new_inventory.name.to_owned(),
        location: new_inventory.location.to_owned(),
        ipaddress: new_inventory.ipaddress.to_owned(),
        hardware: new_inventory.hardware.to_owned(),
    };

    let inventory_detail = db.create_inventory(data).await;
    match inventory_detail {
        Ok(inventory) => HttpResponse::Ok().json(inventory),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/inventory/{id}")]
pub async fn get_inventory(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();

    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }

    let inventory_detail = db.get_inventory(&id).await;
    match inventory_detail {
        Ok(inventory) => HttpResponse::Ok().json(inventory),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/inventory/{id}")]
pub async fn update_inventory(
    db: Data<MongoRepo>,
    path: Path<String>,
    new_inventory: Json<Inventory>,
) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };

    let data = Inventory {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        name: new_inventory.name.to_owned(),
        location: new_inventory.location.to_owned(),
        hardware: new_inventory.hardware.to_owned(),
        ipaddress: new_inventory.ipaddress.to_owned(),
    };

    let update_result = db.update_inventory(&id, data).await;
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_inventory_info = db.get_inventory(&id).await;
                return match updated_inventory_info {
                    Ok(inventory) => HttpResponse::Ok().json(inventory),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                };
            } else {
                return HttpResponse::NotFound().body("No inventory found with specified ID");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/inventory/{id}")]
pub async fn delete_inventory(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();

    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };

    let result = db.delete_inventory(&id).await;
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return HttpResponse::Ok().json("Inventory successfully deleted!");
            } else {
                return HttpResponse::NotFound().json("Inventory with specified ID not found!");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/inventory")]
pub async fn get_all_inventory(db: Data<MongoRepo>) -> HttpResponse {
    let inventory = db.get_all_inventory().await;
    match inventory {
        Ok(inventory) => HttpResponse::Ok().json(inventory),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
