//!
//! Common compiler utilities.
//!

///
/// Deserializes a `serde_json` object from slice with the recursion limit disabled.
///
/// Must be used for all JSON I/O to avoid crashes due to the aforementioned limit.
///
pub fn deserialize_from_slice<O>(input: &[u8]) -> anyhow::Result<O>
where
    O: serde::de::DeserializeOwned,
{
    let deserializer = serde_json::Deserializer::from_slice(input);
    deserialize(deserializer)
}

///
/// Deserializes a `serde_json` object from string with the recursion limit disabled.
///
/// Must be used for all JSON I/O to avoid crashes due to the aforementioned limit.
///
pub fn deserialize_from_str<O>(input: &str) -> anyhow::Result<O>
where
    O: serde::de::DeserializeOwned,
{
    let deserializer = serde_json::Deserializer::from_str(input);
    deserialize(deserializer)
}

///
/// Deserializes a `serde_json` object from reader with the recursion limit disabled.
///
/// Must be used for all JSON I/O to avoid crashes due to the aforementioned limit.
///
pub fn deserialize_from_reader<R, O>(reader: R) -> anyhow::Result<O>
where
    R: std::io::Read,
    O: serde::de::DeserializeOwned,
{
    let deserializer = serde_json::Deserializer::from_reader(reader);
    deserialize(deserializer)
}

///
/// Runs the generic deserializer.
///
pub fn deserialize<'de, R, O>(mut deserializer: serde_json::Deserializer<R>) -> anyhow::Result<O>
where
    R: serde_json::de::Read<'de>,
    O: serde::de::DeserializeOwned,
{
    deserializer.disable_recursion_limit();
    let deserializer = serde_stacker::Deserializer::new(&mut deserializer);
    let result = O::deserialize(deserializer)?;
    Ok(result)
}
