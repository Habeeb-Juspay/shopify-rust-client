use crate::{
    common::{
        http::execute_graphql,
        types::{APIError, RequestCallbacks},
    },
    types::discount::{
        DiscountAutomaticAppCreateResp, DiscountAutomaticAppInput, DiscountAutomaticAppUpdateInput,
        DiscountAutomaticAppUpdateResp, DiscountNodesResp,
    },
};
use serde_json::json;

pub async fn create_automatic_app_discount(
    shop_url: &String,
    version: &String,
    access_token: &String,
    callbacks: &RequestCallbacks,
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

    execute_graphql(shop_url, version, access_token, callbacks, query, variables).await
}

pub async fn update_automatic_app_discount(
    shop_url: &String,
    version: &String,
    access_token: &String,
    callbacks: &RequestCallbacks,
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

    execute_graphql(shop_url, version, access_token, callbacks, query, variables).await
}

pub async fn list_discounts(
    shop_url: &String,
    version: &String,
    access_token: &String,
    callbacks: &RequestCallbacks,
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

    execute_graphql(
        shop_url,
        version,
        access_token,
        callbacks,
        query_str,
        variables,
    )
    .await
}
