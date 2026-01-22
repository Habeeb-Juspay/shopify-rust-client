use crate::{
    common::{
        http::execute_graphql,
        types::{APIError, RequestCallbacks},
    },
    types::app_installation::{
        DeleteMetafieldResp, GetCurrentAppInstallationResp, GetMetafieldResp, ListMetafieldsResp,
        MetafieldInput, SetMetafieldsResp,
    },
};
use serde_json::json;

pub async fn get_current_app_installation(
    shop_url: &String,
    version: &String,
    access_token: &String,
    callbacks: &RequestCallbacks,
) -> Result<GetCurrentAppInstallationResp, APIError> {
    let query = r#"
        query {
            currentAppInstallation {
                id
                accessScopes {
                    handle
                }
                activeSubscriptions {
                    id
                    name
                    status
                }
                launchUrl
            }
        }
    "#
    .to_string();

    let variables = json!({});

    execute_graphql(shop_url, version, access_token, callbacks, query, variables).await
}

pub async fn set_metafields(
    shop_url: &String,
    version: &String,
    access_token: &String,
    callbacks: &RequestCallbacks,
    metafields: Vec<MetafieldInput>,
) -> Result<SetMetafieldsResp, APIError> {
    let query = r#"
        mutation MetafieldsSet($metafields: [MetafieldsSetInput!]!) {
            metafieldsSet(metafields: $metafields) {
                metafields {
                    id
                    namespace
                    key
                    value
                    type
                    createdAt
                    updatedAt
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
        "metafields": metafields
    });

    execute_graphql(shop_url, version, access_token, callbacks, query, variables).await
}

pub async fn get_metafield(
    shop_url: &String,
    version: &String,
    access_token: &String,
    callbacks: &RequestCallbacks,
    app_installation_id: &str,
    namespace: &str,
    key: &str,
) -> Result<GetMetafieldResp, APIError> {
    let query = r#"
        query GetMetafield($id: ID!, $namespace: String!, $key: String!) {
            appInstallation(id: $id) {
                metafield(namespace: $namespace, key: $key) {
                    id
                    namespace
                    key
                    value
                    type
                    createdAt
                    updatedAt
                }
            }
        }
    "#
    .to_string();

    let variables = json!({
        "id": app_installation_id,
        "namespace": namespace,
        "key": key
    });

    execute_graphql(shop_url, version, access_token, callbacks, query, variables).await
}

pub async fn list_metafields(
    shop_url: &String,
    version: &String,
    access_token: &String,
    callbacks: &RequestCallbacks,
    app_installation_id: &str,
    first: Option<i32>,
) -> Result<ListMetafieldsResp, APIError> {
    let query = r#"
        query ListMetafields($id: ID!, $first: Int) {
            appInstallation(id: $id) {
                metafields(first: $first) {
                    edges {
                        node {
                            id
                            namespace
                            key
                            value
                            type
                            createdAt
                            updatedAt
                        }
                    }
                }
            }
        }
    "#
    .to_string();

    let variables = json!({
        "id": app_installation_id,
        "first": first.unwrap_or(10)
    });

    execute_graphql(shop_url, version, access_token, callbacks, query, variables).await
}

pub async fn delete_metafield(
    shop_url: &String,
    version: &String,
    access_token: &String,
    callbacks: &RequestCallbacks,
    metafield_id: &str,
) -> Result<DeleteMetafieldResp, APIError> {
    let query = r#"
        mutation MetafieldDelete($input: MetafieldDeleteInput!) {
            metafieldDelete(input: $input) {
                deletedId
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
            "id": metafield_id
        }
    });

    execute_graphql(shop_url, version, access_token, callbacks, query, variables).await
}
