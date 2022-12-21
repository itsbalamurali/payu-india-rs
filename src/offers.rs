use crate::PayuApiClient;

pub struct Offers {
    client: &'static PayuApiClient
}

impl Offers {
    pub fn new(payu_api_client: &PayuApiClient) -> Self {
        Self {
            client: payu_api_client
        }
    }
}