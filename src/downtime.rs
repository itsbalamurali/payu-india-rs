use crate::PayuApiClient;

pub struct Downtime {
    client: &'static PayuApiClient
}

impl Downtime {
    pub fn new(payu_api_client: &PayuApiClient) -> Self {
        Self {
            client: payu_api_client
        }
    }
}