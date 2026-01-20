use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct CartTransformCreateResp {
    #[serde(rename = "cartTransformCreate")]
    pub cart_transform_create: CartTransformCreatePayload,
}

#[derive(Deserialize, Debug)]
pub struct CartTransformCreatePayload {
    #[serde(rename = "cartTransform")]
    pub cart_transform: Option<CartTransform>,
    #[serde(rename = "userErrors")]
    pub user_errors: Vec<CartTransformCreateUserError>,
}

#[derive(Deserialize, Debug)]
pub struct CartTransform {
    pub id: String,
    #[serde(rename = "functionId")]
    pub function_id: String,
    #[serde(rename = "blockOnFailure")]
    pub block_on_failure: bool,
}

#[derive(Deserialize, Debug)]
pub struct CartTransformCreateUserError {
    pub field: Option<Vec<String>>,
    pub message: String,
    pub code: Option<CartTransformCreateUserErrorCode>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CartTransformCreateUserErrorCode {
    Invalid,
    Taken,
}

#[derive(Serialize, Debug, Clone)]
pub struct CartTransformCreateInput {
    #[serde(rename = "functionHandle")]
    pub function_handle: Option<String>,
    #[serde(rename = "blockOnFailure")]
    pub block_on_failure: Option<bool>,
    pub metafields: Option<Vec<MetafieldInput>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct MetafieldInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub metafield_type: Option<String>,
}

impl CartTransformCreateInput {
    pub fn new() -> Self {
        CartTransformCreateInput {
            function_handle: None,
            block_on_failure: None,
            metafields: None,
        }
    }

    pub fn with_function_handle(mut self, function_handle: String) -> Self {
        self.function_handle = Some(function_handle);
        self
    }

    pub fn with_block_on_failure(mut self, block_on_failure: bool) -> Self {
        self.block_on_failure = Some(block_on_failure);
        self
    }

    pub fn with_metafields(mut self, metafields: Vec<MetafieldInput>) -> Self {
        self.metafields = Some(metafields);
        self
    }
}

impl Default for CartTransformCreateInput {
    fn default() -> Self {
        Self::new()
    }
}

impl MetafieldInput {
    pub fn new(namespace: String, key: String, value: String, metafield_type: String) -> Self {
        MetafieldInput {
            id: None,
            namespace: Some(namespace),
            key: Some(key),
            value: Some(value),
            metafield_type: Some(metafield_type),
        }
    }
}
