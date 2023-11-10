use mollie_api_rust::{
    config::Config,
    payments::create::create_payment,
    payments::create::PaymentRequest, 
    payments::create::Amount,
    payments::create::Metadata,
};

#[tokio::main]
async fn main() {
    // Load configuration.
    let config = Config::new();

    // Create a payment.
    let payment_request = PaymentRequest {
        amount: Amount {
            currency: "EUR".to_string(),
            value: "10.00".to_string(),
        },
        description: "Order #12345".to_string(),
        redirect_url: "https://webshop.example.org/order/12345/".to_string(),
        webhook_url: "https://webshop.example.org/payments/webhook/".to_string(),
        metadata: Metadata {
            order_id: "12345".to_string(),
        },
    };

        // Perform the payment creation.
        match create_payment(&config, payment_request).await {
            Ok(response) => {
                // Handle success, response would typically have details about the payment
                println!("Payment successfully created: {:?}", response);
            },
            Err(e) => {
                // Handle error, e would typically be some error information
                eprintln!("Error creating payment: {:?}", e);
            },
        };
}
