use crate::PayuApiClient;
use anyhow::{anyhow, Error};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Serialize, Deserialize)]
pub enum QrType {
    #[default]
    #[serde(rename = "upi")]
    Upi,
    #[serde(rename = "bqr")]
    BQR,
}

#[derive(Serialize, Deserialize)]
pub struct CreateBharatQR {
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    #[serde(rename = "transactionAmount")]
    pub transaction_amount: String,
    #[serde(rename = "merchantVpa")]
    pub merchant_vpa: String,
    #[serde(rename = "expiryTime")]
    pub expiry_time: String,
    #[serde(rename = "qrType")]
    pub qr_type: QrType,
    #[serde(rename = "qrName")]
    pub qr_name: String,
    #[serde(rename = "qrCity")]
    pub qr_city: String,
    #[serde(rename = "qrPinCode")]
    pub qr_pin_code: String,
    #[serde(rename = "customerName")]
    pub customer_name: String,
    #[serde(rename = "customerCity")]
    pub customer_city: String,
    #[serde(rename = "customerPinCode")]
    pub customer_pin_code: String,
    #[serde(rename = "customerPhone")]
    pub customer_phone: String,
    #[serde(rename = "customerEmail")]
    pub customer_email: String,
    #[serde(rename = "customerAddress")]
    pub customer_address: String,
    pub udf3: String,
    pub udf4: String,
    pub udf5: String,
    #[serde(rename = "outputType")]
    pub output_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct BharatQRCode {
    #[serde(rename = "qrString")]
    pub qr_string: String,
}

pub struct BharatQR {
    client: PayuApiClient,
}

impl BharatQR {
    pub fn new(client: PayuApiClient) -> Self {
        Self { client }
    }

    pub async fn generate_dynamic_bharat_qr(
        self,
        req: &CreateBharatQR,
    ) -> Result<BharatQRCode, Error> {
        let request = serde_json::to_string(req).unwrap();
        let input_vars: HashMap<String, String> = HashMap::from([
            (
                "command".to_string(),
                "generate_dynamic_bharat_qr".to_string(),
            ),
            ("var1".to_string(), request),
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
            Ok(r) => Ok(r.json::<BharatQRCode>().await?),
            Err(e) => Err(anyhow!(e)),
        };
    }
}
