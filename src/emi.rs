use crate::PayuApiClient;

pub struct Emi {
    client: PayuApiClient,
}

impl Emi {
    pub fn new(client: PayuApiClient) -> Self {
        Self { client }
    }
}
