use crate::{
    common::{
        http::execute_graphql,
        types::{APIError, RequestCallbacks},
    },
    types::shopify_functions::ShopifyFunctionsResp,
};
use serde_json::json;

pub async fn list_shopify_functions(
    shop_url: &String,
    version: &String,
    access_token: &String,
    callbacks: &RequestCallbacks,
) -> Result<ShopifyFunctionsResp, APIError> {
    let query = r#"
        query shopifyFunctions {
            shopifyFunctions(first: 25) {
                nodes {
                    id
                    title
                    description
                    apiType
                    apiVersion
                    handle
                    app {
                        id
                        title
                    }
                }
            }
        }
    "#
    .to_string();

    let variables = json!({});

    execute_graphql(shop_url, version, access_token, callbacks, query, variables).await
}
