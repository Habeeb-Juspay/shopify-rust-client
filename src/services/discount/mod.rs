pub mod remote;

use std::sync::Arc;

use crate::{
    common::types::{APIError, RequestCallbacks},
    types::discount::{
        DiscountAutomaticAppCreateResp, DiscountAutomaticAppInput, DiscountAutomaticAppUpdateInput,
        DiscountAutomaticAppUpdateResp, DiscountNodesResp,
    },
};

pub struct Discount {
    pub shop_url: Arc<String>,
    pub version: Arc<String>,
    pub access_token: Arc<String>,
    pub callbacks: Arc<RequestCallbacks>,
}

impl Discount {
    pub fn new(
        shop_url: Arc<String>,
        version: Arc<String>,
        access_token: Arc<String>,
        callbacks: Arc<RequestCallbacks>,
    ) -> Self {
        Discount {
            shop_url,
            version,
            access_token,
            callbacks,
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
            &self.callbacks,
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
            &self.callbacks,
            input,
        )
        .await
    }

    pub async fn list_discounts(
        &self,
        first: Option<i32>,
        after: Option<String>,
        query: Option<String>,
    ) -> Result<DiscountNodesResp, APIError> {
        remote::list_discounts(
            &self.shop_url,
            &self.version,
            &self.access_token,
            &self.callbacks,
            first,
            after,
            query,
        )
        .await
    }
}
