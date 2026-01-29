// Response wrapper for the shop query
#[derive(serde::Deserialize, Debug)]
pub struct GetShopResp {
    pub shop: Shop,
}

// Main Shop struct with owner-related fields
#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Shop {
    pub id: String,
    pub name: String,
    pub email: String,
    pub shop_owner_name: String,
    pub contact_email: String,
    pub myshopify_domain: String,
    pub primary_domain: Domain,
    pub account_owner: StaffMember,
    pub plan: ShopPlan,
    #[serde(rename = "billingAddress")]
    pub shop_address: ShopAddress,
}

#[derive(serde::Deserialize, Debug)]
pub struct StaffMember {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct Domain {
    pub id: String,
    pub host: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShopPlan {
    pub display_name: String,
    pub partner_development: bool,
    pub shopify_plus: bool,
}

#[derive(serde::Deserialize, Debug)]
pub struct ShopAddress {
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub country: Option<String>,
    pub zip: Option<String>,
    pub phone: Option<String>,
    pub company: Option<String>,
}
