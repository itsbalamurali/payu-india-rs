use crate::PayuApiClient;
use anyhow::{anyhow, Error};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct NetBankingStatus {
    pub channel: ChannelStatus,
}

#[derive(Serialize, Deserialize)]
pub struct ChannelStatus {
    #[serde(rename = "ibibo_code")]
    pub code: String,
    pub title: String,
    #[serde(rename = "up_status")]
    pub up_status: i64,
    pub mode: String,
}

#[derive(Serialize, Deserialize)]
pub struct IssuingBankStatus {
    pub issuing_bank: String,
    pub up_status: i64,
}

#[derive(Serialize, Deserialize)]
pub struct DownBins {
    #[serde(rename = "issuing_bank")]
    pub issuing_bank: String,
    pub status: i64,
    pub title: String,
    #[serde(rename = "bins_arr")]
    pub bins_arr: Vec<String>,
}

pub struct Downtime {
    client: PayuApiClient,
}

impl Downtime {
    pub fn new(client: PayuApiClient) -> Self {
        Self { client }
    }

    //Get the netbanking status.
    pub async fn get_netbanking_status(self) -> Result<NetBankingStatus, Error> {
        let input_vars: HashMap<&str, &str> =
            HashMap::from([("command", "getNetbankingStatus"), ("var1", "default")]);
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
            Ok(r) => Ok(r.json::<NetBankingStatus>().await?),
            Err(e) => Err(anyhow!(e)),
        };
    }
    //Get card issuing bank status.
    pub async fn get_issuing_bank_status(
        self,
        bin: &'static str,
    ) -> Result<IssuingBankStatus, Error> {
        let input_vars: HashMap<&str, &str> =
            HashMap::from([("command", "getIssuingBankStatus"), ("var1", bin)]);
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
            Ok(r) => Ok(r.json::<IssuingBankStatus>().await?),
            Err(e) => Err(anyhow!(e)),
        };
    }

    //Get issuing bank card bins which are down.
    pub async fn get_issuing_bank_down_bins(self) -> Result<DownBins, Error> {
        let input_vars: HashMap<&str, &str> =
            HashMap::from([("command", "getIssuingBankDownBins"), ("var1", "default")]);
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
            Ok(r) => Ok(r.json::<DownBins>().await?),
            Err(e) => Err(anyhow!(e)),
        };
    }
}
