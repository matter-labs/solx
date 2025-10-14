//!
//! The test input storage data.
//!

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::str::FromStr;

use crate::directories::matter_labs::test::metadata::case::input::storage::Storage as MatterLabsTestContractStorage;
use crate::test::case::input::value::Value;
use crate::test::instance::Instance;

///
/// The test input storage data.
///
#[derive(Debug, Clone, Default)]
pub struct Storage {
    /// The inner storage hashmap data.
    pub inner: HashMap<web3::types::Address, HashMap<web3::types::U256, web3::types::U256>>,
}

impl Storage {
    ///
    /// Try convert from Matter Labs compiler test storage data.
    ///
    pub fn try_from_matter_labs(
        storage: HashMap<String, MatterLabsTestContractStorage>,
        instances: &BTreeMap<String, Instance>,
    ) -> anyhow::Result<Self> {
        let mut result = HashMap::new();

        for (address, contract_storage) in storage.into_iter() {
            let address = if let Some(instance) = address.strip_suffix(".address") {
                instances
                    .get(instance)
                    .ok_or_else(|| anyhow::anyhow!("Instance `{instance}` not found"))?
                    .address()
                    .copied()
                    .ok_or_else(|| {
                        anyhow::anyhow!("Instance `{instance}` is not successfully deployed")
                    })
            } else {
                web3::types::Address::from_str(address.as_str())
                    .map_err(|error| anyhow::anyhow!("Invalid address literal: {error}"))
            }
            .map_err(|error| anyhow::anyhow!("Invalid storage address: {error}"))?;

            let contract_storage = match contract_storage {
                MatterLabsTestContractStorage::List(list) => list
                    .into_iter()
                    .enumerate()
                    .map(|(key, value)| (key.to_string(), value))
                    .collect(),
                MatterLabsTestContractStorage::Map(map) => map.clone(),
            };
            let mut contract_storage_values = HashMap::new();
            for (key, value) in contract_storage.into_iter() {
                let key = match Value::try_from_matter_labs(key.as_str(), instances)
                    .map_err(|error| anyhow::anyhow!("Invalid storage key: {error}"))?
                {
                    Value::Known(value) => value,
                    Value::Any => anyhow::bail!("Storage key can not be `*`"),
                };

                let value = match Value::try_from_matter_labs(value.as_str(), instances)
                    .map_err(|error| anyhow::anyhow!("Invalid storage value: {error}"))?
                {
                    Value::Known(value) => value,
                    Value::Any => anyhow::bail!("Storage value can not be `*`"),
                };

                let mut value_bytes = [0u8; solx_utils::BYTE_LENGTH_FIELD];
                value.to_big_endian(value_bytes.as_mut_slice());
                let value = web3::types::U256::from(value_bytes);

                contract_storage_values.insert(key, value);
            }
            result.insert(address, contract_storage_values);
        }

        Ok(Self { inner: result })
    }
}
