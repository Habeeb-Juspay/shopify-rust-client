// region: Response Types

#[derive(serde::Deserialize, Debug)]
pub struct DiscountAutomaticAppCreateResp {
    #[serde(rename = "discountAutomaticAppCreate")]
    pub discount_automatic_app_create: DiscountAutomaticAppCreatePayload,
}

#[derive(serde::Deserialize, Debug)]
pub struct DiscountAutomaticAppUpdateResp {
    #[serde(rename = "discountAutomaticAppUpdate")]
    pub discount_automatic_app_update: DiscountAutomaticAppUpdatePayload,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountAutomaticAppCreatePayload {
    pub automatic_app_discount: Option<DiscountAutomaticApp>,
    pub user_errors: Vec<DiscountUserError>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountAutomaticAppUpdatePayload {
    pub automatic_app_discount: Option<DiscountAutomaticApp>,
    pub user_errors: Vec<DiscountUserError>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountAutomaticApp {
    pub discount_id: String,
    pub title: String,
    pub starts_at: String,
    pub ends_at: Option<String>,
    pub status: String,
    pub app_discount_type: AppDiscountType,
    pub combines_with: Option<DiscountCombinesWith>,
    pub applies_on_one_time_purchase: Option<bool>,
    pub applies_on_subscription: Option<bool>,
    pub recurring_cycle_limit: Option<i32>,
    pub metafields: Option<MetafieldConnection>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppDiscountType {
    pub app_key: String,
    pub function_id: String,
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountCombinesWith {
    pub order_discounts: bool,
    pub product_discounts: bool,
    pub shipping_discounts: bool,
}

#[derive(serde::Deserialize, Debug)]
pub struct DiscountUserError {
    pub field: Option<Vec<String>>,
    pub message: String,
    pub code: Option<String>,
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
pub struct Metafield {
    pub id: String,
    pub namespace: String,
    pub key: String,
    pub value: String,
    #[serde(rename = "type")]
    pub metafield_type: String,
}

// endregion

// region: Request Types

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DiscountAutomaticAppInput {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_handle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starts_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub combines_with: Option<DiscountCombinesWithInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_classes: Option<Vec<DiscountClass>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<DiscountContextInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metafields: Option<Vec<MetafieldInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applies_on_subscription: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applies_on_one_time_purchase: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_cycle_limit: Option<i32>,
}

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DiscountAutomaticAppUpdateInput {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_handle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starts_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub combines_with: Option<DiscountCombinesWithInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_classes: Option<Vec<DiscountClass>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<DiscountContextInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metafields: Option<Vec<MetafieldInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applies_on_subscription: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applies_on_one_time_purchase: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_cycle_limit: Option<i32>,
}

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DiscountCombinesWithInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_discounts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_discounts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_discounts: Option<bool>,
}

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DiscountClass {
    Product,
    Order,
    Shipping,
}

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DiscountContextInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<DiscountBuyerSelection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customers: Option<DiscountCustomersInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_segments: Option<DiscountCustomerSegmentsInput>,
}

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DiscountBuyerSelection {
    All,
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct DiscountCustomersInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<String>>,
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct DiscountCustomerSegmentsInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<String>>,
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct MetafieldInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub metafield_type: Option<String>,
}

// endregion

// region: Request Builders

impl DiscountAutomaticAppInput {
    pub fn new(title: String) -> Self {
        DiscountAutomaticAppInput {
            title,
            function_handle: None,
            starts_at: None,
            ends_at: None,
            combines_with: None,
            discount_classes: None,
            context: None,
            metafields: None,
            applies_on_subscription: None,
            applies_on_one_time_purchase: None,
            recurring_cycle_limit: None,
        }
    }

    pub fn with_function_handle(mut self, function_handle: String) -> Self {
        self.function_handle = Some(function_handle);
        self
    }

    pub fn with_starts_at(mut self, starts_at: String) -> Self {
        self.starts_at = Some(starts_at);
        self
    }

    pub fn with_ends_at(mut self, ends_at: String) -> Self {
        self.ends_at = Some(ends_at);
        self
    }

    pub fn with_combines_with(mut self, combines_with: DiscountCombinesWithInput) -> Self {
        self.combines_with = Some(combines_with);
        self
    }

    pub fn with_discount_classes(mut self, discount_classes: Vec<DiscountClass>) -> Self {
        self.discount_classes = Some(discount_classes);
        self
    }

    pub fn with_context(mut self, context: DiscountContextInput) -> Self {
        self.context = Some(context);
        self
    }

    pub fn with_metafields(mut self, metafields: Vec<MetafieldInput>) -> Self {
        self.metafields = Some(metafields);
        self
    }

    pub fn with_applies_on_subscription(mut self, applies: bool) -> Self {
        self.applies_on_subscription = Some(applies);
        self
    }

    pub fn with_applies_on_one_time_purchase(mut self, applies: bool) -> Self {
        self.applies_on_one_time_purchase = Some(applies);
        self
    }

    pub fn with_recurring_cycle_limit(mut self, limit: i32) -> Self {
        self.recurring_cycle_limit = Some(limit);
        self
    }
}

impl DiscountCombinesWithInput {
    pub fn new() -> Self {
        DiscountCombinesWithInput {
            product_discounts: None,
            order_discounts: None,
            shipping_discounts: None,
        }
    }

    pub fn with_product_discounts(mut self, value: bool) -> Self {
        self.product_discounts = Some(value);
        self
    }

    pub fn with_order_discounts(mut self, value: bool) -> Self {
        self.order_discounts = Some(value);
        self
    }

    pub fn with_shipping_discounts(mut self, value: bool) -> Self {
        self.shipping_discounts = Some(value);
        self
    }
}

impl Default for DiscountCombinesWithInput {
    fn default() -> Self {
        Self::new()
    }
}

impl DiscountContextInput {
    pub fn all() -> Self {
        DiscountContextInput {
            all: Some(DiscountBuyerSelection::All),
            customers: None,
            customer_segments: None,
        }
    }

    pub fn customers(add: Vec<String>) -> Self {
        DiscountContextInput {
            all: None,
            customers: Some(DiscountCustomersInput {
                add: Some(add),
                remove: None,
            }),
            customer_segments: None,
        }
    }

    pub fn customer_segments(add: Vec<String>) -> Self {
        DiscountContextInput {
            all: None,
            customers: None,
            customer_segments: Some(DiscountCustomerSegmentsInput {
                add: Some(add),
                remove: None,
            }),
        }
    }
}

impl MetafieldInput {
    pub fn new(namespace: String, key: String, value: String, metafield_type: String) -> Self {
        MetafieldInput {
            id: None,
            namespace: Some(namespace),
            key: Some(key),
            value: Some(value),
            metafield_type: Some(metafield_type),
        }
    }

    pub fn update(id: String, value: String) -> Self {
        MetafieldInput {
            id: Some(id),
            namespace: None,
            key: None,
            value: Some(value),
            metafield_type: None,
        }
    }
}

impl DiscountAutomaticAppUpdateInput {
    pub fn new(id: String) -> Self {
        DiscountAutomaticAppUpdateInput {
            id,
            title: None,
            function_handle: None,
            starts_at: None,
            ends_at: None,
            combines_with: None,
            discount_classes: None,
            context: None,
            metafields: None,
            applies_on_subscription: None,
            applies_on_one_time_purchase: None,
            recurring_cycle_limit: None,
        }
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn with_function_handle(mut self, function_handle: String) -> Self {
        self.function_handle = Some(function_handle);
        self
    }

    pub fn with_starts_at(mut self, starts_at: String) -> Self {
        self.starts_at = Some(starts_at);
        self
    }

    pub fn with_ends_at(mut self, ends_at: String) -> Self {
        self.ends_at = Some(ends_at);
        self
    }

    pub fn with_combines_with(mut self, combines_with: DiscountCombinesWithInput) -> Self {
        self.combines_with = Some(combines_with);
        self
    }

    pub fn with_discount_classes(mut self, discount_classes: Vec<DiscountClass>) -> Self {
        self.discount_classes = Some(discount_classes);
        self
    }

    pub fn with_context(mut self, context: DiscountContextInput) -> Self {
        self.context = Some(context);
        self
    }

    pub fn with_metafields(mut self, metafields: Vec<MetafieldInput>) -> Self {
        self.metafields = Some(metafields);
        self
    }

    pub fn with_applies_on_subscription(mut self, applies: bool) -> Self {
        self.applies_on_subscription = Some(applies);
        self
    }

    pub fn with_applies_on_one_time_purchase(mut self, applies: bool) -> Self {
        self.applies_on_one_time_purchase = Some(applies);
        self
    }

    pub fn with_recurring_cycle_limit(mut self, limit: i32) -> Self {
        self.recurring_cycle_limit = Some(limit);
        self
    }
}

// endregion
