extern crate actix_web;
extern crate mongodb;
extern crate serde_json;


// Import necessary libraries
use actix_web::{test, App};
use mongodb::{Client, Collection, bson::doc};
use serde_json::json;



// Import the module containing the API endpoint
use api::inventory_api::{
    create_inventory, delete_inventory, get_all_inventory, get_inventory, update_inventory,
};

// Define the tests function
#[actix_web::test]
async fn test_inventory_crud<T>() {
    // Initialize a MongoDB client and collection
    let client = Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
    let collection: Collection<T> = client.database("my_test_db").collection("inventory");

    // Clear the collection before each tests
    collection.delete_many(doc! {}, None).await.unwrap();

    // Initialize the App with the API endpoints
    let mut app = test::init_service(
        App::new()
            .app_data(collection.clone())
            .service(create_inventory)
            .service(get_inventory)
            .service(update_inventory)
            .service(delete_inventory)
            .service(get_all_inventory),
    )
        .await;

    // Test the create_inventory endpoint
    let payload = json!({
        "name": "Test Item",
        "location": "us-east-1",
        "hardware": "AMD64",
        "ipaddress": "8.8.8.8",
    });
    let req = test::TestRequest::post()
        .uri("/inventory")
        .set_json(&payload)
        .to_request();
    let res = test::call_service(&mut app, req).await;
    assert!(res.status().is_success());

    // Test the get_inventory endpoint
    let req = test::TestRequest::get().uri("/inventory/1").to_request();
    let res = test::call_service(&mut app, req).await;
    assert!(res.status().is_success());

    // Test the update_inventory endpoint
    let payload = json!({
        "name": "Test Item",
        "location": "us-east-2",
        "hardware": "AMD64",
        "ipaddress": "20.8.8.8",
    });
    let req = test::TestRequest::put()
        .uri("/inventory/1")
        .set_json(&payload)
        .to_request();
    let res = test::call_service(&mut app, req).await;
    assert!(res.status().is_success());

    // Test the delete_inventory endpoint
    let req = test::TestRequest::delete().uri("/inventory/1").to_request();
    let res = test::call_service(&mut app, req).await;
    assert!(res.status().is_success());
}
