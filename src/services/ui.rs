use crate::types::{Menu, Page, Category, Product};
use rocket::{get, post, State};
use lambda_runtime::Error;
use aws_sdk_dynamodb as ddb;
use rocket::serde::json::Json;

#[get("/menu")]
pub async fn get_menu(db: &State<ddb::Client>) -> Json<Vec<Menu>> {
    Menu::default().into()
}

#[get("/page/<page_handle>")]
pub async fn get_page(page_handle: String, db: &State<ddb::Client>) -> Json<Page> {
    Page::default().into()
}

#[get("/pages")]
pub async fn get_pages(db: &State<ddb::Client>) -> Json<Vec<Page>> {
    vec![Page::default()].into()
}

#[get("/<category_handle>")]
pub async fn get_category(category_handle: String, db: &State<ddb::Client>, table_name: &State<String>) -> Json<Category> {
    let results = db
        .query()
        .table_name(table_name)
        .key_condition_expression(format!("partition_key =: {category_handle}"))
        .send()
        .await?;

    println!("{:?}", results);

    Category::default().into()
}

#[get("/<category_handle>/products")]
pub async fn get_category_products(category_handle: String, db: &State<ddb::Client>) -> Json<Vec<Product>> {
    vec![Product::default()].into()
}

#[get("/categories")]
pub async fn get_categories(db: &State<ddb::Client>) -> Json<Vec<Category>> {
    vec![Category::default()].into()
}