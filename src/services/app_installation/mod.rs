pub mod remote;

use std::sync::Arc;

use crate::{
    common::types::{APIError, RequestCallbacks},
    types::app_installation::{
        DeleteMetafieldsResp, GetCurrentAppInstallationResp, GetMetafieldResp, ListMetafieldsResp,
        MetafieldIdentifierInput, MetafieldInput, SetMetafieldsResp,
    },
};

pub struct AppInstallation {
    pub shop_url: Arc<String>,
    pub version: Arc<String>,
    pub access_token: Arc<String>,
    pub callbacks: Arc<RequestCallbacks>,
}

impl AppInstallation {
    pub fn new(
        shop_url: Arc<String>,
        version: Arc<String>,
        access_token: Arc<String>,
        callbacks: Arc<RequestCallbacks>,
    ) -> Self {
        AppInstallation {
            shop_url,
            version,
            access_token,
            callbacks,
        }
    }

    pub async fn get_current(&self) -> Result<GetCurrentAppInstallationResp, APIError> {
        remote::get_current_app_installation(
            &self.shop_url,
            &self.version,
            &self.access_token,
            &self.callbacks,
        )
        .await
    }

    pub async fn set_metafields(
        &self,
        metafields: Vec<MetafieldInput>,
    ) -> Result<SetMetafieldsResp, APIError> {
        remote::set_metafields(
            &self.shop_url,
            &self.version,
            &self.access_token,
            &self.callbacks,
            metafields,
        )
        .await
    }

    pub async fn get_metafield(
        &self,
        app_installation_id: &str,
        namespace: &str,
        key: &str,
    ) -> Result<GetMetafieldResp, APIError> {
        remote::get_metafield(
            &self.shop_url,
            &self.version,
            &self.access_token,
            &self.callbacks,
            app_installation_id,
            namespace,
            key,
        )
        .await
    }

    pub async fn list_metafields(
        &self,
        app_installation_id: &str,
        first: Option<i32>,
    ) -> Result<ListMetafieldsResp, APIError> {
        remote::list_metafields(
            &self.shop_url,
            &self.version,
            &self.access_token,
            &self.callbacks,
            app_installation_id,
            first,
        )
        .await
    }

    pub async fn delete_metafields(
        &self,
        metafields: &[MetafieldIdentifierInput],
    ) -> Result<DeleteMetafieldsResp, APIError> {
        remote::delete_metafields(
            &self.shop_url,
            &self.version,
            &self.access_token,
            &self.callbacks,
            metafields,
        )
        .await
    }
}
