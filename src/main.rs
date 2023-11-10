use mollie_api_rust::{
    config::Config,
    payments::create::create_payment,
    payments::create::PaymentRequest, 
    payments::create::Amount,
    payments::create::Metadata,
};

#[tokio::main]
async fn main() {
    // Configuratie laden
    let config = Config::new();

    // Een betaling maken
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

    match create_payment(&config, payment_request).await {
        Ok(_) => println!("Betaling succesvol aangemaakt"),
        Err(e) => eprintln!("Fout bij het aanmaken van betaling: {}", e),
    };
}
