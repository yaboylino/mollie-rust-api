use crate::config::Config;
use crate::errors::Result;
use crate::errors::MollieError;
use serde::Serialize;
use serde::Deserialize;
use reqwest::StatusCode;

/// Represents a payment to be processed by Mollie.
///
/// # Example
///
/// ```
/// use mollie_api_rust::payments::create::PaymentRequest;
///
/// ```
#[derive(Debug, Serialize)]
pub struct PaymentRequest {
    #[serde(rename = "amount")]
    pub amount: Amount,
    pub description: String,
    #[serde(rename = "redirectUrl")]
    pub redirect_url: String,
    #[serde(rename = "webhookUrl")]
    pub webhook_url: String,
    pub metadata: Metadata,
}

/// A successful response from the Mollie API for a payment creation.
///
/// This struct is used to deserialize the successful response from the Mollie API when a payment has been created.
///
/// # Example
///
/// ```
/// use mollie_api_rust::payments::create::PaymentResponse;
///
/// // Assuming you have a JSON response from the Mollie API stored in `json_response`
/// // let payment_response: PaymentResponse = serde_json::from_str(&json_response).unwrap();
/// ```
#[derive(Debug, Deserialize)]
pub struct PaymentResponse {
    #[serde(rename = "amount")]
    pub amount: Amount,
    pub description: String,
    #[serde(rename = "redirectUrl")]
    pub redirect_url: String,
    #[serde(rename = "webhookUrl")]
    pub webhook_url: String,
    pub metadata: Metadata,
}

/// A payment object representing a payment to be processed by Mollie.
///
/// This struct includes all the necessary fields required to create or represent a payment via the Mollie API.
///
/// # Fields
///
/// * `amount` - The amount that is required for the payment.
/// * `description` - A description of the payment, visible to the customer.
/// * `redirect_url` - The URL the customer will be redirected to after the payment process.
/// * `webhook_url` - The URL Mollie will call once the payment status changes.
/// * `metadata` - Additional metadata associated with the payment.
///
/// # Example
///
/// ```
/// use mollie_api_rust::payments::create::{Payment, Amount, Metadata};
///
/// let payment = Payment {
///     amount: Amount {
///         currency: "EUR".to_string(),
///         value: "10.00".to_string(),
///     },
///     description: "Your Order Description".to_string(),
///     redirect_url: "https://yourshop.com/redirect".to_string(),
///     webhook_url: "https://yourshop.com/webhook".to_string(),
///     metadata: Metadata {
///         order_id: "12345".to_string(),
///     },
/// };
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Payment {
    #[serde(rename = "amount")]
    pub amount: Amount,
    pub description: String,
    #[serde(rename = "redirectUrl")]
    pub redirect_url: String,
    #[serde(rename = "webhookUrl")]
    pub webhook_url: String,
    pub metadata: Metadata,
}

/// Metadata associated with a payment.
///
/// This struct is used to attach additional information to a payment object.
///
/// # Fields
///
/// * `order_id` - A unique identifier for the order provided by the client.
///
/// # Example
///
/// ```
/// use mollie_api_rust::payments::create::Metadata;
///
/// let metadata = Metadata {
///     order_id: "12345".to_string(),
/// };
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(rename = "orderId")]
    pub order_id: String,
}

/// Represents an amount of money in a specific currency.
///
/// This struct is used to specify an amount for payments and refunds.
///
/// # Fields
///
/// * `currency` - The three-letter ISO currency code representing the currency.
/// * `value` - The string representation of the amount in the given currency.
///   It should be formatted to the correct number of decimal places for the currency.
///
/// # Example
///
/// ```
/// use mollie_api_rust::payments::create::Amount;
///
/// let amount = Amount {
///     currency: "EUR".to_string(),
///     value: "10.00".to_string(),
/// };
/// ```
#[derive(Debug, Serialize, Deserialize)]    
pub struct Amount {
    pub currency: String,
    pub value: String,
}

pub async fn create_payment(config: &Config, payment_request: PaymentRequest) -> Result<PaymentResponse> {
    let client = reqwest::Client::new();
    let res = client.post("https://api.mollie.com/v2/payments")
        .bearer_auth(&config.mollie_api_key)
        .json(&payment_request)
        .send()
        .await?;


        match res.status() {
            StatusCode::CREATED => {
                let payment_response = res.json::<PaymentResponse>().await?;
                Ok(payment_response)
            },
            status => {
                Err(MollieError::HttpError(status))
            },
        }
}
