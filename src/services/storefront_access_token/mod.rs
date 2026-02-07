pub mod remote;

use std::sync::Arc;

use crate::{
    common::types::{APIError, RequestCallbacks},
    types::storefront_access_token::{
        ListStorefrontAccessTokensResp, StorefrontAccessTokenCreateInput,
        StorefrontAccessTokenCreateResp, StorefrontAccessTokenDeleteInput,
        StorefrontAccessTokenDeleteResp,
    },
};

pub struct StorefrontAccessToken {
    pub shop_url: Arc<String>,
    pub version: Arc<String>,
    pub access_token: Arc<String>,
    pub callbacks: Arc<RequestCallbacks>,
}

impl StorefrontAccessToken {
    pub fn new(
        shop_url: Arc<String>,
        version: Arc<String>,
        access_token: Arc<String>,
        callbacks: Arc<RequestCallbacks>,
    ) -> Self {
        StorefrontAccessToken {
            shop_url,
            version,
            access_token,
            callbacks,
        }
    }

    pub async fn list(&self) -> Result<ListStorefrontAccessTokensResp, APIError> {
        remote::list_storefront_access_tokens(
            &self.shop_url,
            &self.version,
            &self.access_token,
            &self.callbacks,
        )
        .await
    }

    pub async fn create(
        &self,
        input: &StorefrontAccessTokenCreateInput,
    ) -> Result<StorefrontAccessTokenCreateResp, APIError> {
        remote::create_storefront_access_token(
            &self.shop_url,
            &self.version,
            &self.access_token,
            &self.callbacks,
            input,
        )
        .await
    }

    pub async fn delete(
        &self,
        input: &StorefrontAccessTokenDeleteInput,
    ) -> Result<StorefrontAccessTokenDeleteResp, APIError> {
        remote::delete_storefront_access_token(
            &self.shop_url,
            &self.version,
            &self.access_token,
            &self.callbacks,
            input,
        )
        .await
    }
}
