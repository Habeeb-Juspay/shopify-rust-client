use crate::{
    common::{
        http::execute_graphql,
        types::{APIError, RequestCallbacks},
    },
    types::shop::GetShopResp,
};
use serde_json::json;

pub async fn get_shop(
    shop_url: &String,
    version: &String,
    access_token: &String,
    callbacks: &RequestCallbacks,
) -> Result<GetShopResp, APIError> {
    let query = r#"
        query {
            shop {
                id
                name
                email
                shopOwnerName
                contactEmail
                myshopifyDomain
                primaryDomain {
                    id
                    host
                }
                accountOwner {
                    id
                    name
                    email
                }
                plan {
                    displayName
                    partnerDevelopment
                    shopifyPlus
                }
                billingAddress {
                    address1
                    address2
                    city
                    province
                    country
                    zip
                    phone
                    company
                }
            }
        }
    "#
    .to_string();

    let variables = json!({});

    execute_graphql(shop_url, version, access_token, callbacks, query, variables).await
}
