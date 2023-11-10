use std::env;
use dotenv::dotenv;

/// Configuration for accessing the Mollie API.
///
/// Set up with your Mollie API key which can be obtained from your Mollie Dashboard.
///
/// # Example
///
/// ```
/// use mollie_api_rust::config::Config;
///
/// ```
pub struct Config {
    pub mollie_api_key: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();

        let mollie_api_key = env::var("MOLLIE_API_KEY")
            .expect("MOLLIE_API_KEY not set in the environment or .env file");

        Config { mollie_api_key }
    }
}
