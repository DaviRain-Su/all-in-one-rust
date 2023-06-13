#[cfg(feature = "serde_json")]
pub fn get_dummy_json() -> serde_json::Value {
    // 在使用时判断依赖项是否开启
    {
        let data = "{\"name\": \"Alice\", \"age\": 30}";
        let parsed: serde_json::Value = serde_json::from_str(data).unwrap();
        // println!("data: {:?}", parsed);
        parsed
    }
}
