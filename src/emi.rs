use crate::PayuApiClient;
use anyhow::anyhow;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmiEligibleBins {}

pub struct Emi {
    client: PayuApiClient,
}

impl Emi {
    pub fn new(client: PayuApiClient) -> Self {
        Self { client }
    }

    pub async fn get_eligible_bins_for_emi(self) -> Result<EmiEligibleBins, anyhow::Error> {
        let input_vars = HashMap::from([
            ("command".to_string(), "eligibleBinsForEMI".to_string()),
            ("var1".to_string(), "default".to_string()),
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
