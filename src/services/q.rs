use crate::types::{Product, UIResponder};
use crate::utils::reconstruct_results;
use aws_sdk_dynamodb as ddb;
use aws_sdk_dynamodb::types::AttributeValue;
use rocket::serde::json::Json;
use rocket::{error, get, State};

#[get("/q/product/search/<val>")]
pub async fn q_search_product(
    val: &str,
    db: &State<ddb::Client>,
    table_name: &State<String>,
) -> UIResponder<Vec<Product>> {
    println!("Incoming request value: {:?}", val);

    let cleaned_val = val.trim_matches('"');

    let results = db
        .query()
        .table_name(table_name.to_string())
        .key_condition_expression("partition_key = :pk_val")
        .expression_attribute_values(":pk_val", AttributeValue::S("PRODUCT".to_string()))
        .filter_expression("contains(description, :name)")
        .expression_attribute_values(":name", AttributeValue::S(cleaned_val.to_string()))
        .send()
        .await;

    match results {
        Ok(res) => match reconstruct_results::<Product>(res) {
            Ok(res) => {
                println!("{:?}", res);
                UIResponder::Ok(Json::from(res))
            }
            Err(err) => UIResponder::Err(error!("{:?}", err)),
        },
        Err(err) => {
            println!("{:?}", err);
            UIResponder::Err(error!("Something went wrong"))
        }
    }
}
