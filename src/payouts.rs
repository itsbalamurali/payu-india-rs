use crate::PayuApiClient;

pub struct Payouts {
    client: &'static PayuApiClient
}

impl Payouts {
    pub fn new(payu_api_client: &PayuApiClient) -> Self {
        Self {
            client: payu_api_client
        }
    }
}