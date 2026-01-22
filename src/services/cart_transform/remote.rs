use crate::{
    common::{
        http::execute_graphql,
        types::{APIError, RequestCallbacks},
    },
    types::cart_transform::{CartTransformCreateInput, CartTransformCreateResp},
};
use serde_json::json;

pub async fn create_cart_transform(
    shop_url: &String,
    version: &String,
    access_token: &String,
    callbacks: &RequestCallbacks,
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

    execute_graphql(shop_url, version, access_token, callbacks, query, variables).await
}
