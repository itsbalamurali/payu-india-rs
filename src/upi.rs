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

    pub async fn validate_vpa(self, vpa: String) -> Result<UpiVpaDetails, Error> {
        let mut vars: HashMap<&str, &str> = HashMap::from([
            ("command", "validateVPA"),
            ("var1", &vpa),
            ("var2", "{\"validateAutoPayVPA\":\"1\"}"),
        ]);
        vars = crate::hasher::generate_hash(
            self.client.merchant_key.as_str(),
            self.client.merchant_salt_v2.as_str(),
            vars,
        )
        .unwrap();
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
