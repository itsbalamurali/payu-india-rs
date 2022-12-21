use crate::PayuApiClient;

pub struct Payouts {
    client: PayuApiClient,
}

impl Payouts {
    pub fn new(client: PayuApiClient) -> Self {
        Self { client }
    }
}
