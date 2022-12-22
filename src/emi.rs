use std::collections::HashMap;
use anyhow::anyhow;
use reqwest::Url;
use crate::PayuApiClient;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmiEligibleBins {

}

pub struct Emi {
    client: PayuApiClient,
}

impl Emi {
    pub fn new(client: PayuApiClient) -> Self {
        Self { client }
    }

    pub async fn get_eligible_bins_for_emi(self) -> Result<EmiEligibleBins, anyhow::Error> {
        let input_vars: HashMap<&str, &str> =
            HashMap::from([
                ("command", "eligibleBinsForEMI"),
                ("var1", "default"),
            ]);
        let vars = self.client.generate_hash(input_vars).unwrap();
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
            Ok(r) => Ok(r.json::<EmiEligibleBins>().await?),
            Err(e) => Err(anyhow!(e)),
        };
    }
}
