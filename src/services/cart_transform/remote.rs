use crate::{
    common::types::APIError,
    types::cart_transform::{CartTransformCreateInput, CartTransformCreateResp},
};
use serde_json::json;

#[derive(serde::Serialize)]
struct GraphQLRequest {
    query: String,
    variables: serde_json::Value,
}

#[derive(serde::Deserialize)]
struct GraphQLResponse<T> {
    data: Option<T>,
    errors: Option<Vec<GraphQLError>>,
}

#[derive(serde::Deserialize, Debug)]
struct GraphQLError {
    message: String,
}

async fn execute_graphql<T: serde::de::DeserializeOwned>(
    shop_url: &String,
    version: &String,
    access_token: &String,
    query: String,
    variables: serde_json::Value,
) -> Result<T, APIError> {
    let endpoint = format!("{}/admin/api/{}/graphql.json", shop_url, version);

    let request_body = GraphQLRequest { query, variables };

    let client = reqwest::Client::new();
    let response = client
        .post(&endpoint)
        .header("X-Shopify-Access-Token", access_token)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await;

    match response {
        Ok(resp) => {
            let graphql_response = resp
                .json::<GraphQLResponse<T>>()
                .await
                .map_err(|_| APIError::FailedToParse)?;

            if let Some(errors) = graphql_response.errors {
                let error_messages: Vec<String> =
                    errors.iter().map(|e| e.message.clone()).collect();
                return Err(APIError::ServerError {
                    errors: error_messages.join(", "),
                });
            }

            graphql_response.data.ok_or(APIError::FailedToParse)
        }
        Err(_) => Err(APIError::NetworkError),
    }
}

pub async fn create_cart_transform(
    shop_url: &String,
    version: &String,
    access_token: &String,
    input: &CartTransformCreateInput,
) -> Result<CartTransformCreateResp, APIError> {
    let query = r#"
        mutation cartTransformCreate($blockOnFailure: Boolean, $functionHandle: String, $metafields: [MetafieldInput!]) {
            cartTransformCreate(blockOnFailure: $blockOnFailure, functionHandle: $functionHandle, metafields: $metafields) {
                cartTransform {
                    id
                    functionId
                    blockOnFailure
                }
                userErrors {
                    field
                    message
                    code
                }
            }
        }
    "#
    .to_string();

    let variables = json!({
        "blockOnFailure": input.block_on_failure,
        "functionHandle": input.function_handle,
        "metafields": input.metafields
    });

    execute_graphql(shop_url, version, access_token, query, variables).await
}
