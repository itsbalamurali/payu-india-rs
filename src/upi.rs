use std::collections::HashMap;
use serde::{Serialize,Deserialize};
use crate::PayuApiClient;

pub const SUPPORTED_UPI_HANDLES: HashMap<&str, Vec<&str>> = HashMap::from([
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
    ("Phonepe", vec!["@ybl"], ["@axl"], ["@ibl"]),
    ("Slice", vec!["@sliceaxis"]),
]);

#[derive(Serialize, Deserialize)]
pub struct UpiVpaDetails {
    status: String,
    vpa: String,
    is_vpavalid: i32,
    payer_account_name: String
}

pub struct Upi {
    client: &'static PayuApiClient
}

impl Upi {
    pub fn new(payu_api_client: &PayuApiClient) -> Self {
        Self{
            client: payu_api_client
        }
    }
    pub async fn validate_vpa(self,vpa: String) -> UpiVpaDetails {

    }
}