pub mod remote;

use crate::{
    common::types::{APIError, RequestCallbacks},
    types::shopify_functions::ShopifyFunctionsResp,
};
use std::sync::Arc;

pub struct ShopifyFunctions {
    pub shop_url: Arc<String>,
    pub version: Arc<String>,
    pub access_token: Arc<String>,
    pub callbacks: Arc<RequestCallbacks>,
}

impl ShopifyFunctions {
    pub fn new(
        shop_url: Arc<String>,
        version: Arc<String>,
        access_token: Arc<String>,
        callbacks: Arc<RequestCallbacks>,
    ) -> Self {
        ShopifyFunctions {
            shop_url,
            version,
            access_token,
            callbacks,
        }
    }

    pub async fn list(&self) -> Result<ShopifyFunctionsResp, APIError> {
        remote::list_shopify_functions(
            &self.shop_url,
            &self.version,
            &self.access_token,
            &self.callbacks,
        )
        .await
    }
}
