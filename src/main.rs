mod types;
mod services;

use services::ui::*;
use services::cart::*;
use services::product::*;
use rocket::{self, Error, routes, State};
use lambda_runtime::tracing;
use aws_sdk_dynamodb as ddb;
use serde::{Deserialize, Serialize};
use lambda_web::{is_running_on_lambda, launch_rocket_on_lambda, LambdaError};


#[rocket::main]
async fn main() -> Result<(), LambdaError> {
    tracing::init_default_subscriber();

    let config = aws_config::load_from_env().await;
    let table_name = std::env::var("TABLE_NAME").unwrap_or(String::from("q-apps-table"));

    let rocket = rocket::build()
        .manage(ddb::Client::new(&config))
        .manage(table_name.as_str())
        .mount("/", routes![
            create_cart, add_to_cart, get_cart, update_cart, remove_from_cart, 
            get_product, get_products,
            get_menu, get_page, get_pages, get_category, get_category_products, get_categories
    ]);
    if is_running_on_lambda() {
        // Launch on AWS Lambda
        launch_rocket_on_lambda(rocket).await?;
    } else {
        // Launch local server
        let _ = rocket.launch().await?;
    }
    
    Ok(())
}
