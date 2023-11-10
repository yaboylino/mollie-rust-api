use {
    config::Config,
    payments::create::create_payment,
    PaymentRequest, Amount,
};

#[tokio::test]
async fn test_create_payment() {
    let config = Config::new();
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

    let create_result = create_payment(&config, payment_request).await;
    assert!(create_result.is_ok(), "Failed to create payment");
}