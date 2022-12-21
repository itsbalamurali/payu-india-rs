use std::collections::HashMap;
use anyhow::{anyhow, Error};
use reqwest::{Method, Response};
use serde::{Serialize,Deserialize};
use crate::PayuApiClient;

#[derive(Serialize, Deserialize)]
pub struct BinInfo {
    status: i32, //0 = Failed,1 = Success
    data: BinInfoData
}

#[derive(Serialize, Deserialize)]
pub struct BinData {
    total_count: i32,
    last: i32,
    next_start: i32,
    bins_data: BinsData
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
    is_otp_on_the_fly: i32
}

pub struct Bin {
    client: &'static PayuApiClient
}

impl Bin {
    pub fn new(payu_api_client: &PayuApiClient) -> Self {
        Self {
            client: payu_api_client
        }
    }

    pub async fn get_bin_info(self,bin_number: i64) -> Result<BinInfo, Error> {
        let vars: HashMap<&str, &str> = HashMap::from(
            [
                ("key", self.client.merchant_key.as_str()),
                ("command", "getBINInfo"),
                ("var1", "1"),
                ("var2", bin_number.to_string().as_str()),
                ("var3", "0"),
                ("var5", "1"),
            ]);
        let client = reqwest::Client::new();
        let req = client
            .post(format!("{}/merchant/postservice?form=2",self.client.base_url).parse()?)
            .form(&vars)
            .send().await;
        return match req {
            Ok(r) => Ok(r.json::<BinInfo>().await?),
            Err(e) => Err(anyhow!(e))
        }
    }
}