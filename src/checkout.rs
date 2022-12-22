use crate::PayuApiClient;
use anyhow::anyhow;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct CheckoutDetails {}

#[derive(Serialize, Deserialize)]
pub struct CheckoutRequest {}

pub struct Checkout {
    client: PayuApiClient,
}

impl Checkout {
    pub fn new(client: PayuApiClient) -> Self {
        Self { client }
    }

    pub async fn get_checkout_details(
        self,
        req: &CheckoutRequest,
    ) -> Result<CheckoutDetails, anyhow::Error> {
        let checkout_request = serde_json::to_string(req).unwrap();
        let vars: HashMap<&str, &str> =
            HashMap::from([("command", "get_checkout_details"), ("var1", &checkout_request)]);

        let client = reqwest::Client::new();
        let req = client
            .post(
                Url::parse(
                    format!("{}/merchant/postservice?form=2", self.client.base_url).as_str(),
                )
                .unwrap(),
            )
            .form(&vars)
            .send()
            .await;
        return match req {
            Ok(r) => Ok(r.json::<CheckoutDetails>().await.unwrap()),
            Err(e) => Err(anyhow!(e)),
        };
    }
}
