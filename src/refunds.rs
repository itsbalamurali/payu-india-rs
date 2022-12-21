use crate::PayuApiClient;

pub struct Refunds {
    client: &'static PayuApiClient
}

impl Refunds {
    pub fn new(payu_api_client: &PayuApiClient) -> Self {
        Self {
            client: payu_api_client
        }
    }
}