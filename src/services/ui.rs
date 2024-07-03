use std::collections::HashMap;
use crate::types::{Menu, Page, Category, Product, UIResponder};
use rocket::{error, get, post, Responder, State};
use aws_sdk_dynamodb as ddb;
use aws_sdk_dynamodb::types::AttributeValue;
use rand::Rng;
use rocket::serde::json::Json;
use crate::services::product::reconstruct_product;

#[get("/menu/<menu_id>")]
pub async fn get_menu(menu_id: &str, db: &State<ddb::Client>) -> UIResponder<Vec<Menu>> {
    return match menu_id {
        "navbar" => {
            UIResponder::Ok(vec![
                Menu {
                    title: "AWS Store".to_string(),
                    path: "/".to_string(),
                },
                Menu {
                    title: "All".to_string(),
                    path: "/search".to_string(),
                },
                Menu {
                    title: "Shirts".to_string(),
                    path: "/search/shirts".to_string(),
                },
                Menu {
                    title: "Stickers".to_string(),
                    path: "/search/stickers".to_string(),
                }
            ].into())
        }
        "footer" => {
            UIResponder::Ok(vec![
                Menu {
                    title: "Home".to_string(),
                    path: "/".to_string(),
                },
                Menu {
                    title: "About".to_string(),
                    path: "/about".to_string(),
                },
                Menu {
                    title: "Terms & Conditions".to_string(),
                    path: "/terms-conditions".to_string(),
                },
                Menu {
                    title: "Privacy Policy".to_string(),
                    path: "/privacy-policy".to_string(),
                },
                Menu {
                    title: "FAQ".to_string(),
                    path: "/faq".to_string()
                }
            ].into())
        }
        _ => {
            UIResponder::Err(error!("Menu not found"))
        }
    }
}

#[get("/page/<page_handle>")]
pub async fn get_page(page_handle: String, db: &State<ddb::Client>) -> UIResponder<Page> {
    UIResponder::Ok(Page::default().into())
}

#[get("/pages")]
pub async fn get_pages(db: &State<ddb::Client>) -> UIResponder<Vec<Page>> {
    UIResponder::Ok(vec![Page::default()].into())
}

#[get("/category/<category_handle>")]
pub async fn get_category(
    category_handle: String,
    db: &State<ddb::Client>,
    table_name: &State<String>
) -> UIResponder<Category> {
    let table_name = table_name.inner();
    let results = db
        .query()
        .table_name(table_name)
        .key_condition_expression("partition_key = :pk_val AND begins_with(sort_key, :sk_val)")
        .expression_attribute_values(":pk_val", AttributeValue::S("CATEGORY".to_string()))
        .expression_attribute_values(":sk_val", AttributeValue::S(category_handle.clone()))
        .send()
        .await;

    let results = results.unwrap().items;

    println!("{:?}", results);

    match results {
        Some(items) => {
            // ensure that items only has one item in it
            if items.len() > 1 {
                UIResponder::Err(error!("More than one item found for this category"))
            } else {
                let item = items.get(0).unwrap();
                println!("{:?}", item);

                let category = Category {
                    path: category_handle,
                    category_id: item.get("id").unwrap().as_s().unwrap().to_string(),
                    title: item.get("name").unwrap().as_s().unwrap().to_string(),
                    description : item.get("name").unwrap().as_s().unwrap().to_string(),
                };

                UIResponder::Ok(category.into())
            }

        },
        None => UIResponder::Err(error!("Looks like this category doesn't exist"))
    }
}

#[get("/category/<category_handle>/products")]
pub async fn get_category_products(
    category_handle: &str,
    db: &State<ddb::Client>,
    table_name: &State<String>
) -> UIResponder<Vec<Product>> {
    let table_name = table_name.inner();

    let results = db
        .query()
        .table_name(table_name)
        .key_condition_expression("partition_key = :prod")
        .expression_attribute_values(":prod", AttributeValue::S("PRODUCT".to_string()))
        .filter_expression("category = :category_name")
        .expression_attribute_values(":category_name", AttributeValue::S(category_handle.to_string()))
        .send()
        .await;

    let results = results.unwrap().items;

    let mut products: Vec<Product> = Vec::new();

    // assemble variants and variant images

    for item in results.unwrap() {
        products.push(
            reconstruct_product(
                item.get("id").unwrap().as_s().unwrap().as_str(),
                db.inner(),
                table_name
            )
        );
        println!("{:?}", item);
    }

    UIResponder::Ok(products.into())
}

#[get("/categories")]
pub async fn get_categories(
    db: &State<ddb::Client>,
    table_name: &State<String>
) -> UIResponder<Vec<Category>> {
    let table_name = table_name.inner();

    let results = db
        .query()
        .table_name(table_name)
        .key_condition_expression("partition_key = :pk_val")
        .expression_attribute_values(":pk_val", AttributeValue::S("CATEGORY".to_string()))
        .send()
        .await;

    let results = results.unwrap().items;

    let mut categories: Vec<Category> = Vec::new();

    for item in results.unwrap() {
        let category = Category {
            path: item.get("name").unwrap().as_s().unwrap().to_string(),
            category_id: item.get("id").unwrap().as_s().unwrap().to_string(),
            title: item.get("name").unwrap().as_s().unwrap().to_string(),
            description: item.get("name").unwrap().as_s().unwrap().to_string(),
        };

        categories.push(category.into());
        println!("{:?}", item);
    }

    UIResponder::Ok(categories.into())
}

#[get("/collection/<collection_handle>")]
pub async fn get_collection(
    collection_handle: &str,
    db: &State<ddb::Client>,
    table_name: &State<&str>
) -> UIResponder<Vec<Product>> {
    let table_name = table_name.inner();

    let results = db
        .query()
        .table_name(table_name)
        .key_condition_expression("partition_key = :pk_val AND sort_key = :sk_val")
        .expression_attribute_values(":pk_val", AttributeValue::S("COLLECTION".to_string()))
        .expression_attribute_values(":sk_val", AttributeValue::S(collection_handle.to_string()))
        .send()
        .await;

    let results = results.unwrap().items;

    // if there's more than one result throw error
    // because something is wrong

    match results {
        Some(items) => {
            // ensure that items only has one item in it
            if items.len() > 1 {
                UIResponder::Err(error!("More than one collection found"))
            } else {

            }

        },
        None => UIResponder::Err(error!("Looks like this collection doesn't exist"))
    }
}