use crate::PayuApiClient;

pub struct Invoice {
    client: PayuApiClient,
}

impl Invoice {
    pub fn new(client: PayuApiClient) -> Self {
        Self { client }
    }
}
