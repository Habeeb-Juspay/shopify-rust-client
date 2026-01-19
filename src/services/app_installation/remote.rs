use crate::{
    common::types::APIError,
    types::app_installation::{
        DeleteMetafieldResp, GetCurrentAppInstallationResp, GetMetafieldResp, ListMetafieldsResp,
        MetafieldInput, SetMetafieldsResp,
    },
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

pub async fn get_current_app_installation(
    shop_url: &String,
    version: &String,
    access_token: &String,
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

    execute_graphql(shop_url, version, access_token, query, variables).await
}

pub async fn set_metafields(
    shop_url: &String,
    version: &String,
    access_token: &String,
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

    execute_graphql(shop_url, version, access_token, query, variables).await
}

pub async fn get_metafield(
    shop_url: &String,
    version: &String,
    access_token: &String,
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

    execute_graphql(shop_url, version, access_token, query, variables).await
}

pub async fn list_metafields(
    shop_url: &String,
    version: &String,
    access_token: &String,
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

    execute_graphql(shop_url, version, access_token, query, variables).await
}

pub async fn delete_metafield(
    shop_url: &String,
    version: &String,
    access_token: &String,
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

    execute_graphql(shop_url, version, access_token, query, variables).await
}
