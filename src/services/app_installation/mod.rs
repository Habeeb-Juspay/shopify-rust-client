pub mod remote;

use std::sync::Arc;

use crate::{
    common::types::APIError,
    types::app_installation::{
        DeleteMetafieldResp, GetCurrentAppInstallationResp, GetMetafieldResp, ListMetafieldsResp,
        MetafieldInput, SetMetafieldsResp,
    },
};

pub struct AppInstallation {
    pub shop_url: Arc<String>,
    pub version: Arc<String>,
    pub access_token: Arc<String>,
}

impl AppInstallation {
    pub fn new(shop_url: Arc<String>, version: Arc<String>, access_token: Arc<String>) -> Self {
        AppInstallation {
            shop_url,
            version,
            access_token,
        }
    }

    pub async fn get_current(&self) -> Result<GetCurrentAppInstallationResp, APIError> {
        remote::get_current_app_installation(&self.shop_url, &self.version, &self.access_token)
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
            app_installation_id,
            first,
        )
        .await
    }

    pub async fn delete_metafield(
        &self,
        metafield_id: &str,
    ) -> Result<DeleteMetafieldResp, APIError> {
        remote::delete_metafield(
            &self.shop_url,
            &self.version,
            &self.access_token,
            metafield_id,
        )
        .await
    }
}
