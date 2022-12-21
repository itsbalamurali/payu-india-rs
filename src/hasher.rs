use sha2::Sha512;

pub fn generate_hash(merchant_key:String,merchant_salt_v2:String, command:String, var:String) {
    let mut hasher = Sha512::new();
    hasher.update(format!("{}|{}|{}|{}",merchant_key,command,var,merchant_salt_v2));
    return hasher.finalize();
}

pub fn validate_hash() {

}