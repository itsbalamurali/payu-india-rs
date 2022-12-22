use anyhow::anyhow;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub enum PaymentMode {
    //Net Banking
    NB,
    // Credit Card
    CC,
    DC,       // Debit Card
    CASH,     // Cash Card or Wallet
    UPI,      // UPI,
    EMI,      // EMI,
    NEFTRTGS, //NEFT/RTGS
}

use crate::PayuApiClient;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentDetails {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewPaymentRequest {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentRedirectIntent {
    pub params: String,
    pub post_url: String,
}

pub struct Payments {
    client: PayuApiClient,
}

impl Payments {
    pub fn new(client: PayuApiClient) -> Self {
        Self { client }
    }

    pub fn get_payment_redirect_url(
        self,
        _req: &NewPaymentRequest,
    ) -> Result<PaymentRedirectIntent, anyhow::Error> {
        let vars: HashMap<&str, &str> = HashMap::from([("key", self.client.merchant_key)]);
        let params: Vec<String> = vars.iter().map(|v| format!("{}={}", *v.0, *v.1)).collect();
        return Ok(PaymentRedirectIntent {
            post_url: format!("{}/_payment", self.client.base_url),
            params: params.join("&"),
        });
    }

    pub async fn verify_payment(self, txn_id: String) -> Result<PaymentDetails, anyhow::Error> {
        let vars: HashMap<&str, &str> =
            HashMap::from([("command", "verify_payment"), ("var1", &txn_id)]);

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
            Ok(r) => Ok(r.json::<PaymentDetails>().await.unwrap()),
            Err(e) => Err(anyhow!(e)),
        };
    }

    pub fn get_wallet_codes() -> HashMap<&'static str, &'static str> {
        let wallet_codes: HashMap<&str, &str> = HashMap::from([]);
        return wallet_codes;
    }

    pub fn get_netbanking_codes() -> HashMap<&'static str, &'static str> {
        let netbanking_codes: HashMap<&str, &str> = HashMap::from([
            ("Airtel Payments Bank", "AIRNB"),
            ("Axis NB", "AXIB"),
            ("AU Small Finance Bank", "AUSFNB"),
            ("Bank Of India", "BOIB"),
            ("Bank Of Maharashtra", "BOMB"),
            ("Bharat Co-Op Bank", "BHNB"),
            ("Canara Bank", "CABB"),
            ("Catholic Syrian Bank", "CSBN"),
            ("Central Bank of India", "CBIB"),
            ("City Union Bank", "CUBB"),
            ("Corporation Bank", "CRBP"),
            ("Cosmos Bank", "CSMSNB"),
            ("Dena Bank", "DENN"),
            ("Deutsche Bank", "DSHB"),
            ("Development Credit Bank", "DCBB"),
            ("Dhanlaxmi Bank", "DLSB"),
            ("HDFC Bank", "HDFB"),
            ("ICICI", "ICIB"),
            ("IDFC", "IDFCNB"),
            ("Indian Bank", "INDB"),
            ("Indian Overseas Bank", "INOB"),
            ("IndusInd Bank", "INIB"),
            ("Industrial Development Bank of India (IDBI)", "IDBB"),
            ("Jammu and Kashmir Bank", "JAKB"),
            ("Janata Sahakari Bank Pune", "JSBNB"),
            ("Karnataka Bank", "KRKB"),
            ("Karur Vysya - Corporate Netbanking", "KRVBC"),
            ("Kotak Mahindra Bank", "162B"),
            ("Lakshmi Vilas Bank - Corporate Netbanking", "LVCB"),
            ("Lakshmi Vilas Bank - Retail Netbanking", "LVRB"),
            ("Nainital Bank", "TBON"),
            ("Oriental Bank of Commerce", "OBCB"),
            ("Punjab And Maharashtra Co-operative Bank Limited", "PMNB"),
            ("Punjab And Sind Bank", "PSBNB"),
            ("Punjab National Bank - Corporate Banking", "CPNB"),
            ("Punjab National Bank - Retail Banking", "PNBB"),
            ("Saraswat bank", "SRSWT"),
            ("SBI Netbanking", "SBIB"),
            ("Shamrao Vithal Co-operative Bank Ltd", "SVCNB"),
            ("Syndicate Bank", "SYNDB"),
            ("Tamilnad Mercantile Bank", "TMBB"),
            ("The Federal Bank", "FEDB"),
            ("The Karur Vysya Bank", "KRVB"),
            ("The South Indian Bank", "SOIB"),
            ("UCO Bank", "UCOB"),
            ("Union Bank - Corporate Netbanking", "UBIBC"),
            ("Union Bank Of India", "UBIB"),
            ("Vijaya Bank", "VJYB"),
        ]);
        return netbanking_codes;
    }
}
