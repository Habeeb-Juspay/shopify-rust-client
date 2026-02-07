use serde::{Deserialize, Serialize};

use crate::common::types::{AccessScope, PageInfo, UserError};

// List response wrapper
#[derive(Deserialize, Debug)]
pub struct ListStorefrontAccessTokensResp {
    pub shop: ListStorefrontAccessTokensShop,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListStorefrontAccessTokensShop {
    pub storefront_access_tokens: StorefrontAccessTokenConnection,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StorefrontAccessTokenConnection {
    pub edges: Vec<StorefrontAccessTokenEdge>,
    pub page_info: PageInfo,
}

#[derive(Deserialize, Debug)]
pub struct StorefrontAccessTokenEdge {
    pub node: StorefrontAccessToken,
}

// Core storefront access token type
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StorefrontAccessToken {
    pub id: String,
    pub access_token: String,
    pub access_scopes: Vec<AccessScope>,
    pub created_at: String,
    pub title: String,
    pub updated_at: Option<String>,
}

// Create mutation response
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StorefrontAccessTokenCreateResp {
    pub storefront_access_token_create: StorefrontAccessTokenCreatePayload,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StorefrontAccessTokenCreatePayload {
    pub storefront_access_token: Option<StorefrontAccessToken>,
    pub user_errors: Vec<UserError>,
}

// Delete mutation response
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StorefrontAccessTokenDeleteResp {
    pub storefront_access_token_delete: StorefrontAccessTokenDeletePayload,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StorefrontAccessTokenDeletePayload {
    pub deleted_storefront_access_token_id: Option<String>,
    pub user_errors: Vec<UserError>,
}

// Input types
#[derive(Serialize, Debug, Clone)]
pub struct StorefrontAccessTokenCreateInput {
    pub title: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct StorefrontAccessTokenDeleteInput {
    pub id: String,
}
