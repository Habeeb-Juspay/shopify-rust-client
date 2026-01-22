pub mod remote;

use std::sync::Arc;

use crate::{
    common::types::APIError,
    types::discount::{
        DiscountAutomaticAppCreateResp, DiscountAutomaticAppInput, DiscountAutomaticAppUpdateInput,
        DiscountAutomaticAppUpdateResp,
    },
};

pub struct Discount {
    pub shop_url: Arc<String>,
    pub version: Arc<String>,
    pub access_token: Arc<String>,
}

impl Discount {
    pub fn new(shop_url: Arc<String>, version: Arc<String>, access_token: Arc<String>) -> Self {
        Discount {
            shop_url,
            version,
            access_token,
        }
    }

    pub async fn create_automatic_app_discount(
        &self,
        input: &DiscountAutomaticAppInput,
    ) -> Result<DiscountAutomaticAppCreateResp, APIError> {
        remote::create_automatic_app_discount(
            &self.shop_url,
            &self.version,
            &self.access_token,
            input,
        )
        .await
    }

    pub async fn update_automatic_app_discount(
        &self,
        input: &DiscountAutomaticAppUpdateInput,
    ) -> Result<DiscountAutomaticAppUpdateResp, APIError> {
        remote::update_automatic_app_discount(
            &self.shop_url,
            &self.version,
            &self.access_token,
            input,
        )
        .await
    }
}
