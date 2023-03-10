use crate::PayuApiClient;
use anyhow::{anyhow, Error};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct UpiVpaDetails {
    pub status: String,
    pub vpa: String,
    #[serde(rename = "isVPAValid")]
    pub is_vpa_valid: i64,
    #[serde(rename = "isAutoPayVPAValid")]
    pub is_auto_pay_vpa_valid: i64,
    #[serde(rename = "isAutoPayBankValid")]
    pub is_auto_pay_bank_valid: String,
    #[serde(rename = "payerAccountName")]
    pub payer_account_name: String,
}

pub struct Upi {
    client: PayuApiClient,
}

impl Upi {
    pub fn new(client: PayuApiClient) -> Self {
        Self { client }
    }

    //Get SI/Auto deduct Mandate supported UPI Handles
    pub fn supported_upi_handles() -> HashMap<&'static str, Vec<&'static str>> {
        let supported_upi_handles: HashMap<&str, Vec<&str>> = HashMap::from([
            ("Amazon Pay", vec!["@apl"]),
            ("AXIS Bank", vec!["@axisbank"]),
            ("BHIM", vec!["@upi"]),
            ("BOI (Bank of India)", vec!["@boi"]),
            ("Canara Bank", vec!["@cnrb"]),
            ("Dakpay", vec!["@postbank"]),
            (
                "DBS (The Development Bank of Singapore Limited)",
                vec!["@dbs"],
            ),
            ("Dhanlaxmi Bank", vec!["@dlb"]),
            ("GPay", vec!["@okhdfcbank, @okaxis, @okicici"]),
            ("ICICI iMobile", vec!["@icici"]),
            ("Indus Pay", vec!["@indus"]),
            ("MobiKwik", vec!["@ikwik"]),
            ("PayTM", vec!["@paytm"]),
            ("Payzapp", vec!["@pz"]),
            ("Phonepe", vec!["@ybl", "@axl", "@ibl"]),
            ("Slice", vec!["@sliceaxis"]),
        ]);
        return supported_upi_handles;
    }

    pub async fn validate_vpa(self, vpa: &'static str) -> Result<UpiVpaDetails, Error> {
        let input_vars = HashMap::from([
            ("command".to_string(), "validateVPA".to_string()),
            ("var1".to_string(), vpa.to_string()),
            (
                "var2".to_string(),
                "{\"validateAutoPayVPA\":\"1\"}".to_string(),
            ),
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
            Ok(r) => Ok(r.json::<UpiVpaDetails>().await?),
            Err(e) => Err(anyhow!(e)),
        };
    }
}
