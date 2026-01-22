pub mod remote;

use std::sync::Arc;

use crate::{
    common::types::{APIError, RequestCallbacks},
    types::order::{GetOrderResp, OrderQueryResp, PatchOrderRequest},
};

pub struct Order {
    pub shop_url: Arc<String>,
    pub version: Arc<String>,
    pub access_token: Arc<String>,
    pub callbacks: Arc<RequestCallbacks>,
}

impl Order {
    pub fn new(
        shop_url: Arc<String>,
        version: Arc<String>,
        access_token: Arc<String>,
        callbacks: Arc<RequestCallbacks>,
    ) -> Self {
        Order {
            shop_url,
            version,
            access_token,
            callbacks,
        }
    }

    pub async fn get_with_id(&self, order_id: &String) -> Result<GetOrderResp, APIError> {
        remote::get_order_with_id(
            &self.shop_url,
            &self.version,
            &self.access_token,
            &self.callbacks,
            order_id,
        )
        .await
    }

    pub async fn get_with_name(&self, order_name: &String) -> Result<OrderQueryResp, APIError> {
        remote::get_order_with_name(
            &self.shop_url,
            &self.version,
            order_name,
            &self.access_token,
            &self.callbacks,
        )
        .await
    }

    pub async fn patch(
        &self,
        order_id: &String,
        patch_request: &PatchOrderRequest,
    ) -> Result<GetOrderResp, APIError> {
        remote::patch_order(
            &self.shop_url,
            &self.version,
            &self.access_token,
            &self.callbacks,
            order_id,
            patch_request,
        )
        .await
    }
}
