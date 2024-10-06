#[derive(serde::Deserialize, Debug)]
pub struct OrderQueryResp {
    pub orders: Vec<Order>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct GetOrderResp {
    pub order: Order,
}

#[derive(serde::Deserialize, Debug)]
pub struct ErrorResp {
    pub errors: String,
}

#[derive(serde::Deserialize, Debug, Clone, serde::Serialize)]
pub struct Order {
    pub id: u128,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub line_items: Vec<LineItem>,
    pub fulfillment_status: Option<String>,
    pub fulfillments: Vec<Fulfillment>,
    pub payment_gateway_names: Vec<String>,
    pub subtotal_price_set: PriceSet,
    pub total_discounts_set: PriceSet,
    pub total_price_set: PriceSet,
    pub total_shipping_price_set: PriceSet,
    pub total_tax_set: PriceSet,
    pub order_status_url: Option<String>,
    pub financial_status: Option<String>,
}

#[derive(serde::Deserialize, Debug, Clone, serde::Serialize)]
pub struct Fulfillment {
    pub id: u128,
    pub status: String,
    pub tracking_number: Option<String>,
    pub tracking_company: Option<String>,
    pub tracking_url: Option<String>,
}

#[derive(serde::Deserialize, Debug, Clone, serde::Serialize)]

pub struct LineItem {
    pub id: u128,
    pub quantity: i64,
    pub title: String,
    pub product_id: Option<u128>,
    pub variant_title: Option<String>,
    pub variant_id: u128,
    pub price_set: PriceSet,
    pub requires_shipping: bool,
    pub properties: Vec<Property>,
}

#[derive(serde::Deserialize, Debug, Clone, serde::Serialize)]
pub struct Property {
    pub name: String,
    pub value: String,
}

#[derive(serde::Deserialize, Debug, Clone, serde::Serialize)]
pub struct PriceSet {
    pub shop_money: Money,
    pub presentment_money: Money,
}

#[derive(serde::Deserialize, Debug, Clone, serde::Serialize)]
pub struct Money {
    pub amount: String,
    pub currency_code: String,
}

#[derive(serde::Deserialize, Debug, serde::Serialize)]
pub struct PatchOrderRequest {
    pub order: PatchOrder,
}

#[derive(serde::Deserialize, Debug, serde::Serialize)]
pub struct PatchOrder {
    pub tags: Vec<String>,
}

#[derive(serde::Deserialize, Debug, serde::Serialize)]
pub struct WebhookResponse {
    pub message: String,
}

#[derive(serde::Deserialize, Debug, serde::Serialize, Clone)]
pub enum APIError {
    ServerError { errors: String },
    FailedToParse,
    NetworkError,
}
