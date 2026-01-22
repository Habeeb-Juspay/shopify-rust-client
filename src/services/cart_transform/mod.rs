pub mod remote;

use std::sync::Arc;

use crate::{
    common::types::{APIError, RequestCallbacks},
    types::cart_transform::{CartTransformCreateInput, CartTransformCreateResp},
};

pub struct CartTransform {
    pub shop_url: Arc<String>,
    pub version: Arc<String>,
    pub access_token: Arc<String>,
    pub callbacks: Arc<RequestCallbacks>,
}

impl CartTransform {
    pub fn new(
        shop_url: Arc<String>,
        version: Arc<String>,
        access_token: Arc<String>,
        callbacks: Arc<RequestCallbacks>,
    ) -> Self {
        CartTransform {
            shop_url,
            version,
            access_token,
            callbacks,
        }
    }

    pub async fn create(
        &self,
        input: &CartTransformCreateInput,
    ) -> Result<CartTransformCreateResp, APIError> {
        remote::create_cart_transform(
            &self.shop_url,
            &self.version,
            &self.access_token,
            &self.callbacks,
            input,
        )
        .await
    }
}
