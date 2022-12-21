use crate::PayuApiClient;

pub struct Checkout {
    client: &'static PayuApiClient
}

impl Checkout {
    pub fn new(payu_api_client: &PayuApiClient) -> Self {
        Self {
            client: payu_api_client
        }
    }
}