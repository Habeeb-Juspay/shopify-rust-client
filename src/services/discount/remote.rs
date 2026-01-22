use crate::{
    common::types::APIError,
    types::discount::{
        DiscountAutomaticAppCreateResp, DiscountAutomaticAppInput, DiscountAutomaticAppUpdateInput,
        DiscountAutomaticAppUpdateResp, DiscountNodesResp,
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

pub async fn create_automatic_app_discount(
    shop_url: &String,
    version: &String,
    access_token: &String,
    input: &DiscountAutomaticAppInput,
) -> Result<DiscountAutomaticAppCreateResp, APIError> {
    let query = r#"
        mutation discountAutomaticAppCreate($automaticAppDiscount: DiscountAutomaticAppInput!) {
            discountAutomaticAppCreate(automaticAppDiscount: $automaticAppDiscount) {
                automaticAppDiscount {
                    discountId
                    title
                    startsAt
                    endsAt
                    status
                    appDiscountType {
                        appKey
                        functionId
                        title
                        description
                    }
                    combinesWith {
                        orderDiscounts
                        productDiscounts
                        shippingDiscounts
                    }
                    appliesOnOneTimePurchase
                    appliesOnSubscription
                    recurringCycleLimit
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
        "automaticAppDiscount": input
    });

    execute_graphql(shop_url, version, access_token, query, variables).await
}

pub async fn update_automatic_app_discount(
    shop_url: &String,
    version: &String,
    access_token: &String,
    input: &DiscountAutomaticAppUpdateInput,
) -> Result<DiscountAutomaticAppUpdateResp, APIError> {
    let query = r#"
        mutation discountAutomaticAppUpdate($automaticAppDiscount: DiscountAutomaticAppInput!, $id: ID!) {
            discountAutomaticAppUpdate(automaticAppDiscount: $automaticAppDiscount, id: $id) {
                automaticAppDiscount {
                    discountId
                    title
                    startsAt
                    endsAt
                    status
                    appDiscountType {
                        appKey
                        functionId
                        title
                        description
                    }
                    combinesWith {
                        orderDiscounts
                        productDiscounts
                        shippingDiscounts
                    }
                    appliesOnOneTimePurchase
                    appliesOnSubscription
                    recurringCycleLimit
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
        "id": input.id,
        "automaticAppDiscount": {
            "title": input.title,
            "functionHandle": input.function_handle,
            "startsAt": input.starts_at,
            "endsAt": input.ends_at,
            "combinesWith": input.combines_with,
            "discountClasses": input.discount_classes,
            "context": input.context,
            "metafields": input.metafields,
            "appliesOnSubscription": input.applies_on_subscription,
            "appliesOnOneTimePurchase": input.applies_on_one_time_purchase,
            "recurringCycleLimit": input.recurring_cycle_limit,
        }
    });

    execute_graphql(shop_url, version, access_token, query, variables).await
}

pub async fn list_discounts(
    shop_url: &String,
    version: &String,
    access_token: &String,
    first: Option<i32>,
    after: Option<String>,
    query_filter: Option<String>,
) -> Result<DiscountNodesResp, APIError> {
    let query_str = r#"
        query discountNodes($first: Int, $after: String, $query: String) {
            discountNodes(first: $first, after: $after, query: $query) {
                nodes {
                    id
                    metafields(first: 50) {
                        edges {
                            node {
                                id
                                namespace
                                key
                                value
                                type
                            }
                        }
                    }
                    discount {
                        __typename
                        ... on DiscountAutomaticApp {
                            title
                            status
                            appDiscountType {
                                appKey
                                functionId
                                title
                                description
                            }
                        }
                        ... on DiscountCodeApp {
                            title
                            status
                            appDiscountType {
                                appKey
                                functionId
                                title
                                description
                            }
                        }
                        ... on DiscountAutomaticBasic {
                            title
                            status
                        }
                        ... on DiscountCodeBasic {
                            title
                            status
                        }
                        ... on DiscountAutomaticBxgy {
                            title
                            status
                        }
                        ... on DiscountCodeBxgy {
                            title
                            status
                        }
                        ... on DiscountCodeFreeShipping {
                            title
                            status
                        }
                    }
                }
                pageInfo {
                    hasNextPage
                    hasPreviousPage
                    startCursor
                    endCursor
                }
            }
        }
    "#
    .to_string();

    let variables = json!({
        "first": first.unwrap_or(50),
        "after": after,
        "query": query_filter
    });

    execute_graphql(shop_url, version, access_token, query_str, variables).await
}
