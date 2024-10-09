use aws_sdk_dynamodb as ddb;
use aws_sdk_dynamodb::operation::query::QueryOutput;
use aws_sdk_dynamodb::types::AttributeValue;
use serde::{Deserialize, Serialize};
use serde_dynamo::{from_item, from_items};
use std::fmt::Debug;

pub fn reconstruct_results<'a, T>(results: QueryOutput) -> Result<Vec<T>, String>
where
    T: Debug + Deserialize<'a> + Serialize + Clone,
{
    match from_items(results.items().to_vec()) {
        Ok(inner_res) => Ok(inner_res),
        Err(err) => {
            println!("Error deserializing data: {:?}", err);
            Err("Error deserializing data".to_string())
        }
    }
}

pub fn reconstruct_result<'a, T>(results: QueryOutput) -> Result<T, String>
where
    T: Debug + Deserialize<'a> + Serialize + Clone,
{
    let items: Vec<T> = match from_items(results.items().to_vec()) {
        Ok(inner_res) => inner_res,
        Err(err) => {
            println!("Error deserializing data: {:?}", err);
            return Err("Error deserializing data".to_string());
        }
    };

    if items.len() > 1 {
        return Err("More than one item returned".to_string());
    } else if items.is_empty() {
        return Err("No items returned".to_string());
    }

    // Return the first (and only) item
    Ok(items[0].clone())
}

pub async fn query_ddb(
    table_name: String,
    db: &ddb::Client,
    pk: &str,
    sk: Option<&str>,
) -> Result<QueryOutput, String> {
    let res = match sk {
        Some(sk) => {
            db.query()
                .table_name(table_name)
                .key_condition_expression("partition_key = :pk AND sort_key = :sk")
                .expression_attribute_values(":pk", AttributeValue::S(pk.into()))
                .expression_attribute_values(":sk", AttributeValue::S(sk.into()))
                .send()
                .await
        }
        None => {
            db.query()
                .table_name(table_name)
                .key_condition_expression("partition_key = :pk")
                .expression_attribute_values(":pk", AttributeValue::S(pk.into()))
                .send()
                .await
        }
    };

    match res {
        Ok(res) => Ok(res),
        Err(err) => {
            println!("Error querying DDB: {:?}", err);
            Err(format!(
                "Error querying DDB, {}, {}",
                pk,
                sk.unwrap_or("no sk")
            ))
        }
    }
}
