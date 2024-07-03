use rocket::{error, get, post, State};
use rocket::http::Status;
use rocket::response::status;
use crate::types::{Product, UIResponder};
use lambda_runtime::Error;
use aws_sdk_dynamodb as ddb;
use aws_sdk_dynamodb::types::AttributeValue;
use rand::Rng;
use rocket::serde::json::Json;

#[get("/product/<id>")]
pub async fn get_product(id: &str, db: &State<ddb::Client>) -> UIResponder<Product> {
    todo!()
}

#[post("/products", format="json", data="<search_val>")]
pub async fn get_products(search_val: &str, db: &State<ddb::Client>) -> UIResponder<Vec<Product>> {
    todo!()
}

pub async fn reconstruct_product(product_id: &str, db: &ddb::Client, table_name: &str) -> Product {
    let results = db
        .query()
        .table_name(table_name)
        .key_condition_expression("partition_key = :pk_val")
        .expression_attribute_values(":pk_val", AttributeValue::S("PRODUCT".to_string()))
        .filter_expression("id = :product_id")
        .expression_attribute_values(":product_id", AttributeValue::S(product_id.to_string()))
        .send()
        .await;

    let results = results.unwrap().items;

    println!("{:?}", results);

    match results {
        Some(items) => {
            // if there's more than one item, something is wrong
            if items.len() > 1 {
                Product::default()
            } else {
                let item = items.get(0).unwrap();
                let product = Product {
                    id: item.get("id").unwrap().as_s().unwrap().to_string(),
                    name: item.get("name").unwrap().as_s().unwrap().to_string(),
                    description: item.get("name").unwrap().as_s().unwrap().to_string(),
                    // random number
                    inventory: rand::thread_rng().gen_range(0..100),
                    options: vec![],
                    variants: vec![],
                    price: rand::thread_rng().gen_range(0..300).to_string(),
                    images: vec![],
                };

            }
        }
        None => Product::default()
    }
}