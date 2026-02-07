use crate::{
    common::{
        http::execute_graphql,
        types::{APIError, RequestCallbacks},
    },
    types::storefront_access_token::{
        ListStorefrontAccessTokensResp, StorefrontAccessTokenCreateInput,
        StorefrontAccessTokenCreateResp, StorefrontAccessTokenDeleteInput,
        StorefrontAccessTokenDeleteResp,
    },
};
use serde_json::json;

pub async fn list_storefront_access_tokens(
    shop_url: &String,
    version: &String,
    access_token: &String,
    callbacks: &RequestCallbacks,
) -> Result<ListStorefrontAccessTokensResp, APIError> {
    let query = r#"
        query {
            shop {
                storefrontAccessTokens(first: 100) {
                    edges {
                        node {
                            id
                            accessToken
                            accessScopes {
                                handle
                            }
                            createdAt
                            title
                            updatedAt
                        }
                    }
                    pageInfo {
                        hasNextPage
                        endCursor
                    }
                }
            }
        }
    "#
    .to_string();

    let variables = json!({});

    execute_graphql(shop_url, version, access_token, callbacks, query, variables).await
}

pub async fn create_storefront_access_token(
    shop_url: &String,
    version: &String,
    access_token: &String,
    callbacks: &RequestCallbacks,
    input: &StorefrontAccessTokenCreateInput,
) -> Result<StorefrontAccessTokenCreateResp, APIError> {
    let query = r#"
        mutation storefrontAccessTokenCreate($input: StorefrontAccessTokenInput!) {
            storefrontAccessTokenCreate(input: $input) {
                storefrontAccessToken {
                    id
                    accessToken
                    accessScopes {
                        handle
                    }
                    createdAt
                    title
                }
                userErrors {
                    field
                    message
                }
            }
        }
    "#
    .to_string();

    let variables = json!({
        "input": {
            "title": input.title
        }
    });

    execute_graphql(shop_url, version, access_token, callbacks, query, variables).await
}

pub async fn delete_storefront_access_token(
    shop_url: &String,
    version: &String,
    access_token: &String,
    callbacks: &RequestCallbacks,
    input: &StorefrontAccessTokenDeleteInput,
) -> Result<StorefrontAccessTokenDeleteResp, APIError> {
    let query = r#"
        mutation storefrontAccessTokenDelete($input: StorefrontAccessTokenDeleteInput!) {
            storefrontAccessTokenDelete(input: $input) {
                deletedStorefrontAccessTokenId
                userErrors {
                    field
                    message
                }
            }
        }
    "#
    .to_string();

    let variables = json!({
        "input": {
            "id": input.id
        }
    });

    execute_graphql(shop_url, version, access_token, callbacks, query, variables).await
}
