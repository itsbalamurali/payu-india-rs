use ring::digest::{digest, SHA512};
use std::collections::HashMap;
use std::str::from_utf8;

pub mod bin;
pub mod checkout;
pub mod downtime;
pub mod emi;
pub mod invoice;
pub mod offers;
pub mod payments;
pub mod payouts;
pub mod refunds;
pub mod settlements;
pub mod upi;

#[derive(Debug, Copy, Clone)]
pub struct PayuApiClient {
    pub base_url: &'static str,
    pub merchant_key: &'static str,
    pub merchant_salt_v2: &'static str,
}

pub enum ApiEnv {
    Test,
    Production,
}

impl PayuApiClient {
    pub fn new(env: ApiEnv, merchant_key: &'static str, merchant_salt_v2: &'static str) -> Self {
        return Self {
            base_url: match env {
                ApiEnv::Test => "https://test.payu.in",
                ApiEnv::Production => "https://info.payu.in",
            },
            merchant_key,
            merchant_salt_v2
        };
    }


    fn generate_hash(
        self,
        mut vars: HashMap<&'static str, &'static str>,
    ) -> Result<HashMap<String,String>, anyhow::Error> {
        let command = vars.remove("command").unwrap();
        let var = vars.iter().map(|v| *v.1).collect::<Vec<&str>>().join("|");
        let data = format!(
            "{}|{}|{}|{}",
            self.merchant_key, command, var, self.merchant_salt_v2
        );
        let digest = digest(&SHA512, data.as_bytes());
        let hash = from_utf8(digest.as_ref()).unwrap().to_string();
        let mut map : HashMap<String,String>= HashMap::new();
        map.insert("key".to_string(), self.merchant_key.to_string());
        map.insert("command".to_string(), command.to_string());
        map.insert("hash".to_string(), hash);
        return Ok(map);
    }

    pub fn validate_hash(self, vars: HashMap<&'static str, &'static str>, hash: &'static str) -> bool {
        let gen_hash = self.generate_hash(vars).unwrap();
        let generated_hash = gen_hash.get("hash").unwrap();
        if generated_hash.to_string() == hash.to_string() {
            return true;
        }
        return false;
    }
}
