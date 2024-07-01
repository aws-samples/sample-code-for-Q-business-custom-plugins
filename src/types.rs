use rocket::Responder;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Menu {
    pub title: String,
    pub path: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Page {
    pub id: String,
    pub title: String,
    pub handle: String,
    pub body: String,
    pub body_summary: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Category {
    pub path: String,
    pub category_id: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProductOption {
    pub id: String,
    pub name: String,
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProductVariant {
    pub id: String,
    pub title: String,
    pub price: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Image {
    pub url: String,
    pub alt_text: String,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: String,
    pub inventory: usize,
    pub options: Vec<ProductOption>,
    pub variants: Vec<ProductVariant>,
    pub price: String,
    pub images: Vec<Image>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Cart {
    pub id: String,
    pub products: Vec<Product>,
    pub total_quantity: usize,
    pub cost: String,
    pub checkout_url: String,
}
