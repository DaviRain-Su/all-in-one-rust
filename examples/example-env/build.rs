use serde::{Deserialize, Serialize};
use std::io::Read;

#[derive(Clone, Default, Debug, PartialEq, Deserialize, Serialize)]
pub struct Config {
    pub enabled: bool,
    pub src_chain_id: String,
    pub dst_chain_id: String,
}

fn main() {
    println!("cargo:rustc-env=CURRENT_ID=ibc-0");
    read_counterparty_config("config.toml".to_string());
}

pub fn read_counterparty_config(file_path: String) {
    let mut file = std::fs::File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let config: Config = toml::from_str(&contents).unwrap();

    let counterparty_enable = config.enabled.to_string();
    let src_chain_id = config.src_chain_id;
    let dst_chain_id = config.dst_chain_id;

    println!(
        "cargo:rustc-env=COUNTERPARTY_ENABLE={}",
        counterparty_enable
    );
    println!("cargo:rustc-env=SRC_CHAIN_ID={}", src_chain_id);
    println!("cargo:rustc-env=DST_CHAIN_ID={}", dst_chain_id);
}
