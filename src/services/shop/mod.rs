pub mod remote;

use std::sync::Arc;

use crate::{
    common::types::{APIError, RequestCallbacks},
    types::shop::GetShopResp,
};

pub struct Shop {
    pub shop_url: Arc<String>,
    pub version: Arc<String>,
    pub access_token: Arc<String>,
    pub callbacks: Arc<RequestCallbacks>,
}

impl Shop {
    pub fn new(
        shop_url: Arc<String>,
        version: Arc<String>,
        access_token: Arc<String>,
        callbacks: Arc<RequestCallbacks>,
    ) -> Self {
        Shop {
            shop_url,
            version,
            access_token,
            callbacks,
        }
    }

    pub async fn get(&self) -> Result<GetShopResp, APIError> {
        remote::get_shop(
            &self.shop_url,
            &self.version,
            &self.access_token,
            &self.callbacks,
        )
        .await
    }
}
