//! # Mollie API Rust Client
//!
//! `mollie_api_rust` is a Rust client library for interacting with the [Mollie API](https://docs.mollie.com/).
//! It allows easy integration of Mollie payment services into Rust applications.
//!
//! ## Features
//!
//! - Create, retrieve, update, and delete payments.
//! - Support for asynchronous operations using Rust's async/await.
//! - Strongly typed interfaces for safety and ease of use.
//! - Comprehensive error handling with custom error types.
//!
//! ## Usage
//!
//! Add `mollie_api_rust` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! mollie_api_rust = "0.1.0"
//! ```
//!
//! Set up the client with your Mollie API key and create a payment:
//!
//! ```no_run
//! use mollie_api_rust::Config;
//! use mollie_api_rust::payments::create::{create_payment, PaymentRequest, Amount, Metadata};
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = Config::new();
//!     let payment_request = PaymentRequest {
//!         amount: Amount {
//!             currency: "EUR".to_string(),
//!             value: "10.00".to_string(),
//!         },
//!         description: "Order #12345".to_string(),
//!         redirect_url: "https://yourwebshop.example.org/order/12345/".to_string(),
//!         webhook_url: "https://yourwebshop.example.org/mollie-webhook/".to_string(),
//!         metadata: Metadata {
//!             order_id: "12345".to_string(),
//!         },
//!     };
//!
//!     match create_payment(&config, payment_request).await {
//!         Ok(payment) => println!("Created payment: {:?}", payment),
//!         Err(e) => eprintln!("Failed to create payment: {:?}", e),
//!     }
//! }
//! ```
//!
//! Place your MOLLIE_API_KEY in the .env file.
//!
//! For more information on how to use `mollie_api_rust`, see the examples provided and refer to the [official Mollie API documentation](https://docs.mollie.com/).
//!
//! ## Modules
//!
//! - [`config`](config): Configuration and API client setup.
//! - [`errors`](errors): Custom error types for handling Mollie API errors.
//! - [`payments`](payments): Operations related to payments, such as creating and retrieving payments.
//!

// Crate re-exports
pub use config::Config;
pub use errors::{MollieError, Result};
pub mod payments;
pub mod config;
pub mod errors;
