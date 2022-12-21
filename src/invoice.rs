use crate::PayuApiClient;

pub struct Invoice {
    client: &'static PayuApiClient
}

impl Invoice {
    pub fn new(payu_api_client: &PayuApiClient) -> Self {
        Self {
            client: payu_api_client
        }
    }
}