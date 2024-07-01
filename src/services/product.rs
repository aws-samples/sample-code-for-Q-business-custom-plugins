use rocket::{get, post, State};
use rocket::http::Status;
use rocket::response::status;
use crate::types::{Product};
use lambda_runtime::Error;
use aws_sdk_dynamodb as ddb;
use rocket::serde::json::Json;

#[get("/products/<id>")]
pub async fn get_product(id: String, db: &State<ddb::Client>) -> Json<Product> {
    todo!()
}

#[post("/products", format="json", data="<search_val>")]
pub async fn get_products(search_val: String, db: &State<ddb::Client>) -> Json<Vec<Product>> {
    todo!()
}