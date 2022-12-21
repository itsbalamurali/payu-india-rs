mod bin;
mod checkout;
mod downtime;
mod emi;
mod hasher;
mod invoice;
mod offers;
mod payments;
mod payouts;
mod refunds;
mod settlements;
mod upi;

pub struct PayuApiClient {
    base_url: String,
    merchant_key: String,
    merchant_salt_v2: String,
}

pub enum ApiEnv {
    Test,
    Production,
}

impl PayuApiClient {
    pub fn new(env: ApiEnv, merchant_key: String, merchant_salt_v2: String) -> Self {
        return Self {
            base_url: match env {
                ApiEnv::Test => "https://test.payu.in".to_string(),
                ApiEnv::Production => "https://info.payu.in".to_string(),
            },
            merchant_key,
            merchant_salt_v2,
        };
    }
}
