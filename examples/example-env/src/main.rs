fn main() {
    let counter_party_enable = std::env::var("COUNTERPARTY_ENABLE").unwrap();
    let src_chain_id = std::env::var("SRC_CHAIN_ID").unwrap();
    let dst_chain_id = std::env::var("DST_CHAIN_ID").unwrap();
    println!("export COUNTERPARTY_ENABLE={}", counter_party_enable);
    println!("export SRC_CHAIN_ID={}", src_chain_id);
    println!("export DST_CHAIN_ID={}", dst_chain_id);
}
