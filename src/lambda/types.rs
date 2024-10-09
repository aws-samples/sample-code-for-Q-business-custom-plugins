use rocket::serde::json::Json;
use rocket::Responder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Menu {
    pub title: String,
    pub path: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Page {
    pub id: String,
    pub title: String,
    pub handle: String,
    pub body: String,
    pub body_summary: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Category {
    pub partition_key: String,
    pub sort_key: String,
    pub path: String,
    pub category_id: String,
    pub title: String,
    pub description: String,
    pub products: Vec<Product>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProductOption {
    pub id: String,
    pub name: String,
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProductVariant {
    pub id: String,
    pub title: String,
    pub price: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Image {
    pub url: String,
    pub alt_text: String,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CartProduct {
    pub product: Product,
    pub quantity: usize,
    pub selected_variant: Option<ProductVariant>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Product {
    pub partition_key: String,
    pub sort_key: String,
    pub id: String,
    pub name: String,
    pub description: String,
    pub inventory: usize,
    pub options: Option<Vec<ProductOption>>,
    pub variants: Option<Vec<ProductVariant>>,
    pub price: String,
    pub images: Vec<Image>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Cart {
    pub partition_key: String,
    pub sort_key: String,
    pub id: String,
    pub products: Vec<CartProduct>,
    pub total_quantity: usize,
    pub cost: String,
    pub checkout_url: String,
}

#[derive(Responder)]
pub enum UIResponder<T> {
    Ok(Json<T>),
    #[response(status = 404)]
    Err(()),
}
