# Mollie API Rust Client

This library provides a Rust interface to the [Mollie API](https://docs.mollie.com/) for processing payments. It wraps the API in a convenient Rust package for integration into Rust applications.

## Features

- Asynchronous API requests using Rust's `async/await`.
- Strongly typed structures for compile-time verification of Mollie API requests and responses.
- Comprehensive error handling with custom error types.
- Supports all major payment functionalities provided by Mollie.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
mollie_api_rust = "0.1.0"
```

Then run cargo build to download and compile the library and its dependencies.

Usage
To use the Mollie API Rust client, you first need to set up a configuration with your Mollie API key:

```
use mollie_api_rust::payments::create::{create_payment, PaymentRequest, Amount, Metadata};
use mollie_api_rust::Config;

#[tokio::main]
async fn main() {
    let config = Config::new("your_api_key_here");

    let payment_request = PaymentRequest {
        amount: Amount {
            currency: "EUR".to_string(),
            value: "10.00".to_string(),
        },
        description: "Order #12345".to_string(),
        redirect_url: "https://yourwebshop.example.org/order/12345/".to_string(),
        webhook_url: "https://yourwebshop.example.org/mollie-webhook/".to_string(),
        metadata: Metadata {
            order_id: "12345".to_string(),
        },
    };

    match create_payment(&config, payment_request).await {
        Ok(payment) => println!("Created payment: {:?}", payment),
        Err(e) => eprintln!("Failed to create payment: {:?}", e),
    }
}
```

## Configuration
The library requires an API key to authenticate requests to Mollie. This key can be set in the Config struct as shown in the usage example.

## Documentation
For comprehensive documentation, after adding this crate as a dependency, run:

```
cargo doc --open
```

This command will generate documentation locally and open it in your web browser.

## Contributing
Contributions to this library are welcome, especially in the following areas:

## Adding new API features from Mollie.
Improving existing documentation and code examples.
Writing tests and improving code coverage.
Please feel free to submit issues and pull requests on the repository.

## License
This library is distributed under the terms of the MIT license. See LICENSE for details.
# mollie-rust-api
