use crate::types::{Cart, Product};
use rocket::{delete, get, post, State};
use lambda_runtime::Error;
use aws_sdk_dynamodb as ddb;
use rocket::serde::json::Json;

#[post("/cart/create", format="json", data="<user_id>")]
pub async fn create_cart(user_id: Json<String>, db: &State<ddb::Client>) -> Json<Cart> {
    todo!()
}

#[get("/cart/<cart_id>")]
pub async fn get_cart(cart_id: String, db: &State<ddb::Client>) -> Json<Cart> {
    todo!()
}

#[post("/cart/add", format="json", data="<product_to_add>")]
pub async fn add_to_cart(product_to_add: Json<Product>, db: &State<ddb::Client>) -> Json<Cart> {
    todo!()
}

#[post("/cart/update", format="json", data="<update_cart>")]
pub async fn update_cart(update_cart: Json<Cart>, db: &State<ddb::Client>) -> Json<Cart> {
    todo!()
}

#[post("/cart/<cart_id>", format="json", data="<delete_from_cart>")]
pub async fn remove_from_cart(cart_id: String, delete_from_cart: Json<Product>,db: &State<ddb::Client>) -> Json<Cart> {
    todo!()
}