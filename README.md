# Shopify Rust Client

A Rust client library for interacting with the Shopify Admin API. Supports both REST and GraphQL APIs with a focus on app development and order management.

## Features

### Current Support

#### Core APIs
- 📦 **Orders** (REST): Get orders by ID/name, update order properties
- 💳 **Subscriptions** (GraphQL): Recurring/usage/combined subscriptions, trial management, usage tracking
- 🎁 **Discounts** (GraphQL): Create and manage automatic app discounts
- ⚙️ **App Installation** (GraphQL): App metadata, metafields management
- 🛒 **Cart Transform** (GraphQL): Create cart transformations with metafield support (create & update)
- 🔧 **Shopify Functions** (GraphQL): List available functions
- 🏪 **Shop** (GraphQL): Fetch shop owner details, plan info, and billing address

#### Developer Experience
- 📦 **Type-Safe**: Strongly typed models for all API responses
- 🚀 **Async/Await**: Built on `reqwest` for asynchronous HTTP requests
- 🔐 **Secure**: Token-based authentication support
- 🪝 **Webhook Support**: Parse customer and shop compliance webhooks
- 🔍 **Request Callbacks**: Optional before/after hooks for observability and logging
- 🎯 **Client-Based**: Single client instance with organized service modules

### Roadmap

- 🚧 **Products API**: Product management and variants
- 🚧 **Customers API**: Customer management and search
- 🚧 **Inventory API**: Inventory tracking and locations
- 🚧 **Fulfillments API**: Order fulfillment operations
- 🚧 **Additional Webhooks**: Support for more webhook topics beyond compliance
- 🚧 **Rate Limiting**: Built-in request throttling and retry logic

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
shopify-client = "0.1.0"
```

Or install directly:

```bash
cargo add shopify-client
```

## Public API

The library exposes the following public modules:

- **`ShopifyClient`** - Main client for making API calls
- **`types`** - All type definitions organized by resource (e.g., `types::order`, `types::subscription`)
- **`webhooks`** - Webhook parsing utilities and types
- **Callback Types** - `BeforeRequestCallback`, `AfterRequestCallback`, `RequestCallbacks`

```rust
use shopify_client::ShopifyClient;                    // Main client
use shopify_client::types::order::*;                  // Order types
use shopify_client::types::subscription::*;           // Subscription types
use shopify_client::types::shop::*;                   // Shop types
use shopify_client::webhooks::*;                      // Webhook utilities
use shopify_client::{BeforeRequestCallback, AfterRequestCallback};  // Callback types
```

## Usage

### Initialize the Client

```rust
use shopify_client::ShopifyClient;

let client = ShopifyClient::new(
    "https://your-shop.myshopify.com".to_string(),
    "your-access-token".to_string(),
    None, // Optional API version, defaults to "2026-01"
);
```

### Initialize with Request Callbacks

```rust
use shopify_client::ShopifyClient;
use std::sync::Arc;

let before_request = Arc::new(|url: &str, body: Option<&str>, _headers: &reqwest::header::HeaderMap| {
    println!("→ Request to {}", url);
    if let Some(body) = body {
        println!("  Body: {}", body);
    }
});

let after_request = Arc::new(|url: &str, response: &str, _headers: &reqwest::header::HeaderMap| {
    println!("← Response from {} ({} bytes)", url, response.len());
});

let client = ShopifyClient::new_with_callbacks(
    "https://your-shop.myshopify.com".to_string(),
    "your-access-token".to_string(),
    None,
    Some(before_request),
    Some(after_request),
);
```

### Get Order by ID

```rust
use shopify_client::ShopifyClient;

#[tokio::main]
async fn main() {
    let client = ShopifyClient::new(
        "https://your-shop.myshopify.com".to_string(),
        "your-access-token".to_string(),
        None,
    );

    let order_id = "1234567890".to_string();

    match client.order.get_with_id(&order_id).await {
        Ok(response) => {
            println!("Order: {:?}", response.order);
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}
```

### Get Order by Name

```rust
use shopify_client::ShopifyClient;

#[tokio::main]
async fn main() {
    let client = ShopifyClient::new(
        "https://your-shop.myshopify.com".to_string(),
        "your-access-token".to_string(),
        None,
    );

    let order_name = "1001".to_string(); // Without the # prefix

    match client.order.get_with_name(&order_name).await {
        Ok(response) => {
            println!("Orders found: {}", response.orders.len());
            for order in response.orders {
                println!("Order {}: {}", order.name, order.id);
            }
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}
```

### Update Order (Patch)

```rust
use shopify_client::ShopifyClient;
use shopify_client::types::order::{PatchOrderRequest, PatchOrder};

#[tokio::main]
async fn main() {
    let client = ShopifyClient::new(
        "https://your-shop.myshopify.com".to_string(),
        "your-access-token".to_string(),
        None,
    );

    let order_id = "1234567890".to_string();

    let patch_request = PatchOrderRequest {
        order: PatchOrder {
            tags: vec!["processed".to_string(), "priority".to_string()],
        },
    };

    match client.order.patch(&order_id, &patch_request).await {
        Ok(response) => {
            println!("Order updated: {:?}", response.order);
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}
```

### Create Recurring Subscription

```rust
use shopify_client::ShopifyClient;
use shopify_client::types::subscription::{CreateRecurringSubscriptionRequest, AppPricingInterval};

#[tokio::main]
async fn main() {
    let client = ShopifyClient::new(
        "https://your-shop.myshopify.com".to_string(),
        "your-access-token".to_string(),
        None,
    );

    let request = CreateRecurringSubscriptionRequest {
        name: "Premium Plan".to_string(),
        price: 29.99,
        currency_code: "USD".to_string(),
        return_url: "https://your-app.com/billing".to_string(),
        interval: Some(AppPricingInterval::Every30Days),
        trial_days: Some(7),
        test: Some(true),
        discount: None,
    };

    match client.subscription.create_recurring(&request).await {
        Ok(response) => {
            println!("Subscription created: {:?}", response);
            println!("Confirmation URL: {}", response.confirmation_url);
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}
```

### Create Automatic App Discount

```rust
use shopify_client::ShopifyClient;
use shopify_client::types::discount::DiscountAutomaticAppInput;

#[tokio::main]
async fn main() {
    let client = ShopifyClient::new(
        "https://your-shop.myshopify.com".to_string(),
        "your-access-token".to_string(),
        None,
    );

    let input = DiscountAutomaticAppInput {
        title: "Summer Sale".to_string(),
        function_handle: "my-discount-function".to_string(),
        starts_at: "2024-06-01T00:00:00Z".to_string(),
        ends_at: None,
        combines_with: None,
        discount_classes: None,
        context: None,
        metafields: None,
        applies_on_subscription: None,
        applies_on_one_time_purchase: None,
        recurring_cycle_limit: None,
    };

    match client.discount.create_automatic_app_discount(&input).await {
        Ok(response) => {
            println!("Discount created: {:?}", response);
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}
```

### Create Cart Transform with Metafields

```rust
use shopify_client::ShopifyClient;
use shopify_client::types::cart_transform::{CartTransformCreateInput, MetafieldInput};

#[tokio::main]
async fn main() {
    let client = ShopifyClient::new(
        "https://your-shop.myshopify.com".to_string(),
        "your-access-token".to_string(),
        None,
    );

    // Create metafields to configure the cart transform function
    let metafields = vec![
        MetafieldInput::new(
            "$app".to_string(),
            "config".to_string(),
            r#"{"bundleDiscount": 10, "minQuantity": 2}"#.to_string(),
            "json".to_string(),
        ),
    ];

    let input = CartTransformCreateInput::new()
        .with_function_handle("my-cart-transform".to_string())
        .with_block_on_failure(false)
        .with_metafields(metafields);

    match client.cart_transform.create(&input).await {
        Ok(response) => {
            println!("Cart transform created: {:?}", response);
            if let Some(cart_transform) = response.cart_transform_create.cart_transform {
                println!("ID: {}", cart_transform.id);
                println!("Function ID: {}", cart_transform.function_id);
            }
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}
```

### Update Metafields on Existing Cart Transform

```rust
use shopify_client::ShopifyClient;
use shopify_client::types::cart_transform::MetafieldsSetInput;

#[tokio::main]
async fn main() {
    let client = ShopifyClient::new(
        "https://your-shop.myshopify.com".to_string(),
        "your-access-token".to_string(),
        None,
    );

    let cart_transform_id = "gid://shopify/CartTransform/123456".to_string();

    // Update existing metafield and create a new one
    let metafields = vec![
        MetafieldsSetInput::new(
            cart_transform_id.clone(),
            "$app".to_string(),
            "config".to_string(),
            r#"{"bundleDiscount": 15, "minQuantity": 3}"#.to_string(),
            "json".to_string(),
        ),
        MetafieldsSetInput::new(
            cart_transform_id.clone(),
            "$app".to_string(),
            "settings".to_string(),
            r#"{"enabled": true, "priority": 1}"#.to_string(),
            "json".to_string(),
        ),
    ];

    match client.cart_transform.set_metafields(&metafields).await {
        Ok(response) => {
            println!("Metafields updated successfully");
            if let Some(metafields) = response.metafields_set.metafields {
                for metafield in metafields {
                    println!("  {}.{} = {}", metafield.namespace, metafield.key, metafield.value);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}
```

### Update Metafields with Compare-and-Set (CAS)

For safe concurrent updates, use the `compare_digest` field to ensure the metafield hasn't changed since you last read it:

```rust
use shopify_client::ShopifyClient;
use shopify_client::types::cart_transform::MetafieldsSetInput;

#[tokio::main]
async fn main() {
    let client = ShopifyClient::new(
        "https://your-shop.myshopify.com".to_string(),
        "your-access-token".to_string(),
        None,
    );

    let cart_transform_id = "gid://shopify/CartTransform/123456".to_string();

    // Update with compare-and-set to prevent race conditions
    let metafields = vec![
        MetafieldsSetInput::new(
            cart_transform_id.clone(),
            "$app".to_string(),
            "config".to_string(),
            r#"{"bundleDiscount": 20, "minQuantity": 4}"#.to_string(),
            "json".to_string(),
        )
        .with_compare_digest(Some("fd6b73725c9e83da2d2bcfaf90b27305b9058a48a1565639aa00d718d4caf8e8".to_string())),
    ];

    match client.cart_transform.set_metafields(&metafields).await {
        Ok(response) => {
            println!("Metafields updated with CAS");
            if let Some(metafields) = response.metafields_set.metafields {
                for metafield in metafields {
                    println!("  {}.{} (digest: {:?})", 
                        metafield.namespace, 
                        metafield.key, 
                        metafield.compare_digest
                    );
                }
            }
        }
        Err(e) => {
            eprintln!("Error (may indicate digest mismatch): {:?}", e);
        }
    }
}
```

### Get Shop Owner Details

```rust
use shopify_client::ShopifyClient;

#[tokio::main]
async fn main() {
    let client = ShopifyClient::new(
        "https://your-shop.myshopify.com".to_string(),
        "your-access-token".to_string(),
        None,
    );

    match client.shop.get().await {
        Ok(response) => {
            let shop = response.shop;
            println!("Shop Name: {}", shop.name);
            println!("Owner: {}", shop.shop_owner_name);
            println!("Email: {}", shop.email);
            println!("Contact Email: {}", shop.contact_email);
            println!("Domain: {}", shop.myshopify_domain);
            println!("Primary Domain: {}", shop.primary_domain.host);
            println!("Plan: {}", shop.plan.display_name);
            println!("Account Owner: {} ({})", shop.account_owner.name, shop.account_owner.email);
            if let Some(city) = &shop.shop_address.city {
                println!("Location: {}", city);
            }
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}
```

### Parse Webhooks

```rust
use shopify_client::webhooks::{parse_webhook_with_header, WebhookPayload};

fn handle_webhook(topic_header: &str, payload: &str) {
    match parse_webhook_with_header(topic_header, payload) {
        Ok(WebhookPayload::CustomersDataRequest(data)) => {
            println!("Customer data request for shop: {}", data.shop_domain);
            println!("Customer: {:?}", data.customer);
            println!("Orders requested: {:?}", data.orders_requested);
        }
        Ok(WebhookPayload::CustomersRedact(data)) => {
            println!("Customer redact request for shop: {}", data.shop_domain);
            println!("Customer to redact: {:?}", data.customer);
            println!("Orders to redact: {:?}", data.orders_to_redact);
        }
        Ok(WebhookPayload::ShopRedact(data)) => {
            println!("Shop redact request for shop: {}", data.shop_domain);
            println!("Shop ID: {}", data.shop_id);
        }
        Err(e) => {
            eprintln!("Failed to parse webhook: {:?}", e);
        }
    }
}
```

## Project Structure

The library is organized into a modular structure for easy extensibility:

```
src/
├── lib.rs                  # Main client entry point and exports
├── types/                  # Public type definitions
│   ├── mod.rs             # Type module exports
│   ├── order.rs           # Order types (REST)
│   ├── subscription.rs    # Subscription types (GraphQL)
│   ├── discount.rs        # Discount types (GraphQL)
│   ├── app_installation.rs # App installation types (GraphQL)
│   ├── cart_transform.rs  # Cart transform types (GraphQL)
│   ├── shop.rs            # Shop types (GraphQL)
│   └── shopify_functions.rs # Shopify functions types (GraphQL)
├── webhooks/              # Public webhook parsing module
│   ├── mod.rs             # Webhook parsing functions
│   └── types.rs           # Webhook payload types
├── common/                # Internal shared utilities (private)
│   ├── mod.rs
│   ├── types.rs           # Common types (APIError, RequestCallbacks)
│   ├── utils.rs           # Utility functions
│   └── http.rs            # Centralized GraphQL execution
└── services/              # Internal API services (private)
    ├── order/             # Order service (REST)
    ├── subscription/      # Subscription service (GraphQL)
    ├── discount/          # Discount service (GraphQL)
    ├── app_installation/  # App installation service (GraphQL)
    ├── cart_transform/    # Cart transform service (GraphQL)
    ├── shop/              # Shop service (GraphQL)
    └── shopify_functions/ # Shopify functions service (GraphQL)
        ├── mod.rs         # Service struct with public methods
        └── remote.rs      # Internal API implementation
```

### Design Philosophy

- **Client-Based**: Initialize a `ShopifyClient` once and access services through it
- **Clean Public API**: Three public modules - `ShopifyClient`, `types`, and `webhooks`
- **Encapsulation**: All internal implementation (`common`, `services`) is private
- **Service-Oriented**: Each API resource (orders, products, etc.) is its own service module
- **Clear Separation**: Services for outgoing API calls, webhooks for incoming data parsing
- **Type Safety**: All API responses are strongly typed with comprehensive models

## Data Models

The library provides strongly-typed models organized by resource:

### Order Types (`types::order`)

- **Order**: Complete order information including customer, line items, fulfillments, pricing, and timestamps
- **Customer**: Customer details (ID, email, phone, name, addresses)
- **LineItem**: Product line items with quantities, prices, properties, and fulfillment status
- **OrderFulfillment**: Fulfillment tracking with tracking numbers, URLs, and shipment status
- **Address**: Billing and shipping address information
- **PriceSet**: Multi-currency pricing with shop and presentment money
- **Property**: Custom line item properties
- **PatchOrderRequest** / **PatchOrder**: Request types for updating orders

### Subscription Types (`types::subscription`)

- **CreateRecurringSubscriptionRequest**: Create recurring app subscriptions
- **CreateUsageSubscriptionRequest**: Create usage-based subscriptions
- **CreateCombinedSubscriptionRequest**: Create subscriptions with both recurring and usage pricing
- **CreateUsageRecordRequest**: Record usage for usage-based subscriptions
- **AppPricingInterval**: Subscription billing intervals (Every30Days, Annual)
- **CreateSubscriptionResp**: Subscription creation response with confirmation URL
- **ActiveSubscriptionsResp**: List of active app subscriptions

### Discount Types (`types::discount`)

- **DiscountAutomaticAppInput**: Create automatic app discounts
- **DiscountAutomaticAppUpdateInput**: Update existing automatic app discounts
- **DiscountAutomaticAppCreateResp**: Discount creation response
- **DiscountNodesResp**: List discounts with pagination support

### App Installation Types (`types::app_installation`)

- **GetCurrentAppInstallationResp**: Current app installation details
- **MetafieldInput**: Input for creating/updating metafields
- **SetMetafieldsResp**: Metafield operation response
- **GetMetafieldResp** / **ListMetafieldsResp**: Metafield retrieval responses

### Cart Transform Types (`types::cart_transform`)

- **CartTransformCreateInput**: Create cart transformation functions
- **CartTransformCreateResp**: Cart transform creation response
- **MetafieldInput**: Metafield input for cart transform creation
- **MetafieldsSetInput**: Update or create metafields on existing resources
- **MetafieldsSetResp**: Metafields set response with updated metafield data
- **Metafield**: Metafield data with timestamps and compare digest for CAS

### Shop Types (`types::shop`)

- **GetShopResp**: Response wrapper for the shop query
- **Shop**: Shop information including owner details, plan, and addresses
- **StaffMember**: Account owner details (ID, name, email)
- **Domain**: Primary domain information
- **ShopPlan**: Billing plan details (display name, partner development, Shopify Plus)
- **ShopAddress**: Shop billing address

### Shopify Functions Types (`types::shopify_functions`)

- **ShopifyFunctionsResp**: List of available Shopify Functions

### Webhook Types (`webhooks::types`)

- **CustomersDataRequestPayload**: Customer data request webhook payload
- **CustomersRedactPayload**: Customer redaction webhook payload
- **ShopRedactPayload**: Shop redaction webhook payload
- **WebhookPayload**: Enum containing all webhook payload types
- **WebhookParseError**: Error type for webhook parsing failures
- **WebhookCustomer**: Customer information in webhook payloads

## Error Handling

The library uses a custom `APIError` enum for comprehensive error handling:

```rust
pub enum APIError {
    ServerError { errors: String },
    FailedToParse,
    NetworkError,
}
```

Errors are returned through `Result<T, APIError>` for easy error propagation and handling.

## Authentication

This client requires a Shopify Admin API access token. You can obtain one by:

1. Creating a custom app in your Shopify admin panel
2. Generating an Admin API access token
3. Granting the necessary permissions (e.g., `read_orders`, `write_orders`)

## Request Callbacks

The library supports optional before/after request callbacks for logging, monitoring, and observability:

```rust
use shopify_client::ShopifyClient;
use std::sync::Arc;

let before = Arc::new(|url: &str, body: Option<&str>, headers: &reqwest::header::HeaderMap| {
    println!("Making request to: {}", url);
    if let Some(body) = body {
        println!("Request body: {}", body);
    }
});

let after = Arc::new(|url: &str, response_body: &str, headers: &reqwest::header::HeaderMap| {
    println!("Received response from: {}", url);
    println!("Response size: {} bytes", response_body.len());
});

let client = ShopifyClient::new_with_callbacks(
    "https://your-shop.myshopify.com".to_string(),
    "your-access-token".to_string(),
    None,
    Some(before),
    Some(after),
);
```

**Features:**
- Callbacks are synchronous closures that fire before and after every request
- Access token is NOT passed to callbacks for security
- Callbacks use panic catching to prevent user errors from crashing requests
- Works for both REST and GraphQL requests
- Optional - use `new()` for no callbacks, `new_with_callbacks()` for callbacks

## API Version

Currently uses Shopify Admin API version **2026-01**.

## Requirements

- Rust 1.56 or later (2021 edition)
- Dependencies:
  - `reqwest` (with JSON support)
  - `serde` (with derive feature)
  - `serde_json`

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under your preferred license.

## Disclaimer

This is an unofficial Shopify client library. For official SDKs and documentation, visit [Shopify's Developer Documentation](https://shopify.dev/).
