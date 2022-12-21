use crate::PayuApiClient;

pub struct Settlements {
    client: &'static PayuApiClient
}

impl Settlements {
    pub fn new(payu_api_client: &PayuApiClient) -> Self {
        Self {
            client: payu_api_client
        }
    }
}