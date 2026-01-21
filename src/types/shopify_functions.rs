use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ShopifyFunctionsResp {
    #[serde(rename = "shopifyFunctions")]
    pub shopify_functions: ShopifyFunctionConnection,
}

#[derive(Deserialize, Debug)]
pub struct ShopifyFunctionConnection {
    pub nodes: Vec<ShopifyFunction>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ShopifyFunction {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    #[serde(rename = "apiType")]
    pub api_type: String,
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub app: Option<App>,
    pub handle: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct App {
    pub id: String,
    pub title: String,
}
