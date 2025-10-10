use crate::test::case::input::value::Value;

pub fn web3_u256_to_revm_u256(u256: web3::types::U256) -> revm::primitives::U256 {
    let mut bytes = [0_u8; 32];
    u256.to_big_endian(&mut bytes);
    revm::primitives::U256::from_be_bytes(bytes)
}

pub fn web3_address_to_revm_address(address: &web3::types::Address) -> revm::primitives::Address {
    let bytes: &mut [u8; 32] = &mut [0; 32];
    web3::types::U256::from(address.as_bytes()).to_big_endian(bytes);
    revm::primitives::Address::from_word(revm::primitives::FixedBytes::new(*bytes))
}

pub fn revm_bytes_to_vec_value(bytes: revm::primitives::Bytes) -> Vec<Value> {
    let mut datas = vec![];
    datas.extend_from_slice(&bytes);
    let mut data_value = vec![];
    for data in datas.chunks(32) {
        if data.len() < 32 {
            let mut value = [0u8; 32];
            value[..data.len()].copy_from_slice(data);
            data_value.push(Value::Known(web3::types::U256::from_big_endian(&value)));
        } else {
            let mut value = [0u8; 32];
            value.copy_from_slice(data);
            data_value.push(Value::Known(web3::types::U256::from_big_endian(&value)));
        }
    }
    data_value
}

pub fn revm_topics_to_vec_value(revm_topics: &[revm::primitives::B256]) -> Vec<Value> {
    let mut topics = vec![];
    for topic in revm_topics.iter() {
        let mut topic_value = [0u8; 32];
        topic_value.copy_from_slice(topic.as_slice());
        topics.push(Value::Known(web3::types::U256::from_big_endian(
            &topic_value,
        )));
    }
    topics
}
