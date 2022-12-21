use ring::digest::{digest, SHA512};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::str;
use std::str::from_utf8;

pub fn generate_hash<'a>(
    key: &'a str,
    salt: &'a str,
    mut vars: HashMap<&'a str, &'a str>,
) -> Result<HashMap<&'a str, &'a str>, anyhow::Error> {
    let command = vars.remove("command").unwrap();
    let var = vars.iter().map(|v| *v.1).collect::<Vec<&str>>().join("|");
    let data = format!("{}|{}|{}|{}", key, command, var, salt);
    let hash = digest(&SHA512, data.as_bytes());
    let map: HashMap<&str, &str> = HashMap::from([
        ("key", key),
        ("command", command),
        ("hash", from_utf8(hash.as_ref()).unwrap()),
    ]);
    return Ok(vars);
}

pub fn validate_hash(key: &str, salt: &str, vars: HashMap<&str, &str>, hash: &str) -> bool {
    let gen_hash = generate_hash(key, salt, vars).unwrap();
    let generated_hash = gen_hash.get("hash").unwrap();
    if generated_hash.to_string() == hash.to_string() {
        return true;
    }
    return false;
}
