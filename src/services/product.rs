use rocket::{get, post, State};
use rocket::http::Status;
use rocket::response::status;
use crate::types::{Product, UIResponder};
use lambda_runtime::Error;
use aws_sdk_dynamodb as ddb;
use rocket::serde::json::Json;

#[get("/product/<id>")]
pub async fn get_product(id: String, db: &State<ddb::Client>) -> UIResponder<Product> {
    todo!()
}

#[post("/products", format="json", data="<search_val>")]
pub async fn get_products(search_val: String, db: &State<ddb::Client>) -> UIResponder<Vec<Product>> {
    todo!()
}