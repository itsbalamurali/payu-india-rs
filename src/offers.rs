use crate::PayuApiClient;

pub struct Offers {
    client: PayuApiClient,
}

impl Offers {
    pub fn new(client: PayuApiClient) -> Self {
        Self { client }
    }
}
