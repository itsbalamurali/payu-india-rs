use crate::PayuApiClient;
use anyhow::{anyhow, Error};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct BinInfo {
    status: i32, //0 = Failed,1 = Success
    data: Data,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    total_count: i32,
    last: i32,
    next_start: i32,
    bins_data: BinsData,
}

#[derive(Serialize, Deserialize)]
pub struct BinsData {
    // The issuing bank of the card used for the transaction
    issuing_bank: String,
    // The BIN number of the card is displayed in the response.
    bin: String,
    // Response value can contain any of the following:
    //  creditcard signifies that the particular bin is a credit card BIN
    //  debitcard signifies that the particular bin is a debit card BIN
    category: String,
    // Response value can contain any of the following:
    // MAST
    // VISA
    // MAES
    // AMEX
    // DINR
    // Unknown
    card_type: String,
    // Response value can contain any of the following:
    // 0 signifies that the particular BIN is domestic.
    // 1 signifies that the particular BIN is International.
    is_domestic: i32,
    //Response value can contain any of the following:
    // 0 signifies that the card is not an ATM card.
    // 1 signifies that the card is an ATM card.
    is_atmpin_card: i32,
    // Response value can contain any of the following:
    // 0 signifies that the card does not have OTP on the fly facility.
    // 1 signifies that the card have OTP on the fly facility.
    is_otp_on_the_fly: i32,
}

#[derive(Serialize, Deserialize)]
pub struct BinDetails {
    #[serde(rename = "isDomestic")]
    pub is_domestic: String,
    #[serde(rename = "issuingBank")]
    pub issuing_bank: String,
    #[serde(rename = "cardType")]
    pub card_type: String,
    #[serde(rename = "cardCategory")]
    pub card_category: String,
}

pub struct Bin {
    client: PayuApiClient,
}

impl Bin {
    pub fn new(client: PayuApiClient) -> Self {
        Self { client }
    }

    pub async fn get_bin_info(self, bin: i64) -> Result<BinInfo, Error> {
        let input_vars = HashMap::from([
            ("command".to_string(), "getBINInfo".to_string()),
            ("var1".to_string(), "1".to_string()),
            ("var2".to_string(), bin.to_string()),
            ("var3".to_string(), "0".to_string()),
            ("var5".to_string(), "1".to_string()),
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
            Ok(r) => Ok(r.json::<BinInfo>().await.unwrap()),
            Err(e) => Err(anyhow!(e)),
        };
    }

    pub async fn check_is_domestic(self, bin: i64) -> Result<BinDetails, Error> {
        let input_vars = HashMap::from([
            ("command".to_string(), "check_isDomestic".to_string()),
            ("var1".to_string(), bin.to_string()),
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
            Ok(r) => Ok(r.json::<BinDetails>().await.unwrap()),
            Err(e) => Err(anyhow!(e)),
        };
    }
}
