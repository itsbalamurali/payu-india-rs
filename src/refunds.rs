use crate::PayuApiClient;

pub struct Refunds {
    client: PayuApiClient,
}

impl Refunds {
    pub fn new(client: PayuApiClient) -> Self {
        Self { client }
    }
}
