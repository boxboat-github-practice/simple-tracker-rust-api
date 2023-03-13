use std::env;
extern crate dotenv;
use dotenv::dotenv;
use futures::TryStreamExt;

use crate::models::inventory_model::Inventory;
use mongodb::options::ClientOptions;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};

pub struct MongoRepo {
    col: Collection<Inventory>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client_option = ClientOptions::parse(uri).await;
        let client = Client::with_options(client_option.unwrap());
        let db = client.expect("Could not create DB").database("simple_tracker");
        let col: Collection<Inventory> = db.collection("Inventory");
        MongoRepo { col }
    }

    pub async fn create_inventory(
        &self,
        new_inventory: Inventory,
    ) -> Result<InsertOneResult, Error> {
        let new_doc = Inventory {
            id: None,
            name: new_inventory.name,
            location: new_inventory.location,
            hardware: new_inventory.hardware,
            ipaddress: new_inventory.ipaddress,
        };
        let inventory = self
            .col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating inventory");
        Ok(inventory)
    }

    pub async fn get_inventory(&self, id: &String) -> Result<Inventory, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let inventory_detail = self
            .col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting inventory detail");
        Ok(inventory_detail.unwrap())
    }

    pub async fn update_inventory(
        &self,
        id: &String,
        new_inventory: Inventory,
    ) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_inventory.id,
                    "name": new_inventory.name,
                    "location": new_inventory.location,
                    "hardware": new_inventory.hardware,
                    "ipaddress": new_inventory.ipaddress                    },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating inventory");
        Ok(updated_doc)
    }

    pub async fn delete_inventory(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let inventory_detail = self
            .col
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting inventory");
        Ok(inventory_detail)
    }

    pub async fn get_all_inventory(&self) -> Result<Vec<Inventory>, Error> {
        let mut cursors = self
            .col
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of inventory");
        let mut systems: Vec<Inventory> = Vec::new();
        while let Some(inventory) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            systems.push(inventory)
        }
        Ok(systems)
    }
}
