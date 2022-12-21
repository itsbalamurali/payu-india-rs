use crate::PayuApiClient;

pub struct Emi {
    client: &'static PayuApiClient
}

impl Emi {
    pub fn new(payu_api_client: &PayuApiClient) -> Self {
        Self {
            client: payu_api_client
        }
    }
}