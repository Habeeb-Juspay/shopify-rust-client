use std::sync::Arc;

pub mod common;
pub mod services;
pub mod types;
pub mod webhooks;

pub struct ShopifyClient {
    pub order: services::order::Order,
    pub subscription: services::subscription::Subscription,
    pub app_installation: services::app_installation::AppInstallation,
    pub discount: services::discount::Discount,
    pub cart_transform: services::cart_transform::CartTransform,
    pub shopify_functions: services::shopify_functions::ShopifyFunctions,
}

impl ShopifyClient {
    pub fn new(shop_url: String, access_token: String, api_version: Option<String>) -> Self {
        let api_version = api_version.unwrap_or("2024-07".to_string());
        let shop_url_arc = Arc::new(shop_url);
        let api_version_arc = Arc::new(api_version);
        let access_token_arc = Arc::new(access_token);

        ShopifyClient {
            order: services::order::Order::new(
                Arc::clone(&shop_url_arc),
                Arc::clone(&api_version_arc),
                Arc::clone(&access_token_arc),
            ),
            subscription: services::subscription::Subscription::new(
                Arc::clone(&shop_url_arc),
                Arc::clone(&api_version_arc),
                Arc::clone(&access_token_arc),
            ),
            app_installation: services::app_installation::AppInstallation::new(
                Arc::clone(&shop_url_arc),
                Arc::clone(&api_version_arc),
                Arc::clone(&access_token_arc),
            ),
            discount: services::discount::Discount::new(
                Arc::clone(&shop_url_arc),
                Arc::clone(&api_version_arc),
                Arc::clone(&access_token_arc),
            ),
            cart_transform: services::cart_transform::CartTransform::new(
                Arc::clone(&shop_url_arc),
                Arc::clone(&api_version_arc),
                Arc::clone(&access_token_arc),
            ),
            shopify_functions: services::shopify_functions::ShopifyFunctions::new(
                Arc::clone(&shop_url_arc),
                Arc::clone(&api_version_arc),
                Arc::clone(&access_token_arc),
            ),
        }
    }
}
