use crate::common::types::{APIError, RequestCallbacks};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};

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

pub async fn execute_graphql<T: serde::de::DeserializeOwned>(
    shop_url: &String,
    version: &String,
    access_token: &String,
    callbacks: &RequestCallbacks,
    query: String,
    variables: serde_json::Value,
) -> Result<T, APIError> {
    let endpoint = format!("{}/admin/api/{}/graphql.json", shop_url, version);

    let request_body = GraphQLRequest { query, variables };

    let body_str = serde_json::to_string(&request_body).unwrap_or_default();

    let mut callback_headers = HeaderMap::new();
    callback_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    callbacks.call_before(&endpoint, Some(&body_str), &callback_headers);

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
            let response_headers = resp.headers().clone();
            let response_text = resp.text().await.map_err(|_| APIError::FailedToParse)?;

            callbacks.call_after(&endpoint, &response_text, &response_headers);

            let graphql_response = serde_json::from_str::<GraphQLResponse<T>>(&response_text)
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
