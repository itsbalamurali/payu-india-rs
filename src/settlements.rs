use crate::PayuApiClient;
use anyhow::anyhow;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SettlementDetails {
    pub status: i64,
    pub msg: String,
    #[serde(rename = "Txn_details")]
    pub txn_details: Vec<TxnDetail>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TxnDetail {
    pub payuid: i64,
    pub txnid: i64,
    pub txndate: String,
    pub mode: String,
    pub amount: f64,
    pub requestid: i64,
    pub requestdate: String,
    pub requestaction: String,
    pub requestamount: f64,
    pub mer_utr: String,
    pub mer_service_fee: f64,
    pub mer_service_tax: f64,
    pub mer_net_amount: f64,
    pub bank_name: String,
    pub issuing_bank: String,
    pub merchant_subvention_amount: f64,
    pub cgst: f64,
    pub igst: f64,
    pub sgst: f64,
    #[serde(rename = "PG_TYPE")]
    pub pg_type: String,
    #[serde(rename = "Card Type")]
    pub card_type: String,
    pub token: String,
    #[serde(rename = "SettlementType")]
    pub settlement_type: String,
    #[serde(rename = "PG")]
    pub pg: String,
    #[serde(rename = "Scheme")]
    pub scheme: String,
    #[serde(rename = "FeeType")]
    pub fee_type: String,
    #[serde(rename = "InstantSettlementTDR")]
    pub instant_settlement_tdr: f64,
    #[serde(rename = "InstantSettlementTDRTax")]
    pub instant_settlement_tdrtax: f64,
    #[serde(rename = "InstantSettlementTdrType")]
    pub instant_settlement_tdr_type: String,
    #[serde(rename = "InstantRefundTDR")]
    pub instant_refund_tdr: f64,
    #[serde(rename = "InstantRefundTDRTax")]
    pub instant_refund_tdrtax: f64,
    #[serde(rename = "InstantRefundTdrType")]
    pub instant_refund_tdr_type: String,
}

pub struct Settlements {
    client: PayuApiClient,
}

impl Settlements {
    pub fn new(client: PayuApiClient) -> Self {
        Self { client }
    }

    pub async fn get_settlement_details(
        self,
        date: String,
        utr: Option<String>,
    ) -> Result<SettlementDetails, anyhow::Error> {
        let mut vars: HashMap<&str, &str> = HashMap::from([
            ("command", "get_settlement_details"),
            ("var1", &date),
            ("var2", "5"),
            ("var3", "10000"),
            ("var4", "L"),
            ("var5", "2"),
        ]);
        if utr.is_some() {}
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
            Ok(r) => Ok(r.json::<SettlementDetails>().await.unwrap()),
            Err(e) => Err(anyhow!(e)),
        };
    }
}
