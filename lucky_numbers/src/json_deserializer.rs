//!
//! Rust Lambda
//! Copyright (c) 2019 SilentByte <https://silentbyte.com/>
//!

use serde::de::{Deserialize, DeserializeOwned, Deserializer, Error};
use serde_json;

/// Represents a deserializer function that deserializes values from a JSON string and
/// is intended to be used in conjunction with serde's with attribute, e.g.
/// ```
/// #[derive(Deserialize, Debug)]
/// struct LambdaInput {
///     #[serde(with = "json_deserializer")]
///     body: Payload,
/// }
/// ```
pub fn deserialize<'a, T: DeserializeOwned, D: Deserializer<'a>>(
    deserializer: D,
) -> Result<T, D::Error> {
    serde_json::from_str(&String::deserialize(deserializer)?).map_err(Error::custom)
}
