use crate::common::types::{AccessScope, UserError};

// Response Types

#[derive(serde::Deserialize, Debug)]
pub struct GetCurrentAppInstallationResp {
    #[serde(rename = "currentAppInstallation")]
    pub current_app_installation: AppInstallation,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppInstallation {
    pub id: String,
    pub access_scopes: Vec<AccessScope>,
    pub active_subscriptions: Vec<AppSubscription>,
    pub launch_url: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppSubscription {
    pub id: String,
    pub name: String,
    pub status: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct SetMetafieldsResp {
    #[serde(rename = "metafieldsSet")]
    pub metafields_set: MetafieldsSetPayload,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MetafieldsSetPayload {
    pub metafields: Option<Vec<Metafield>>,
    pub user_errors: Vec<UserError>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Metafield {
    pub id: String,
    pub namespace: String,
    pub key: String,
    pub value: String,
    #[serde(rename = "type")]
    pub metafield_type: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetMetafieldResp {
    pub app_installation: AppInstallationWithMetafield,
}

#[derive(serde::Deserialize, Debug)]
pub struct AppInstallationWithMetafield {
    pub metafield: Option<Metafield>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListMetafieldsResp {
    pub app_installation: AppInstallationWithMetafields,
}

#[derive(serde::Deserialize, Debug)]
pub struct AppInstallationWithMetafields {
    pub metafields: MetafieldConnection,
}

#[derive(serde::Deserialize, Debug)]
pub struct MetafieldConnection {
    pub edges: Vec<MetafieldEdge>,
}

#[derive(serde::Deserialize, Debug)]
pub struct MetafieldEdge {
    pub node: Metafield,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeleteMetafieldsResp {
    pub metafields_delete: MetafieldsDeletePayload,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MetafieldsDeletePayload {
    pub deleted_metafields: Option<Vec<MetafieldIdentifier>>,
    pub user_errors: Vec<UserError>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MetafieldIdentifier {
    pub owner_id: String,
    pub namespace: String,
    pub key: String,
}

// Input type for deleting metafields
#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MetafieldIdentifierInput {
    pub owner_id: String,
    pub namespace: String,
    pub key: String,
}

// Request Types

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MetafieldInput {
    pub namespace: String,
    pub key: String,
    pub value: String,
    #[serde(rename = "type")]
    pub metafield_type: String,
    pub owner_id: String,
}
