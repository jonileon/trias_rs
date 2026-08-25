pub mod request_utils;
pub mod time_format;
pub mod env;
pub mod common_structures;

#[allow(dead_code)]
pub mod xs_duration {
    use chrono::Duration;
    use serde::{Deserialize, Deserializer, Serializer};
    pub fn serialize<S>(value: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("PT{}S", value.num_seconds()))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let seconds = if let Some(v) = s.strip_prefix("PT").and_then(|v| v.strip_suffix('S')) {
            v.parse::<i64>().map_err(serde::de::Error::custom)?
        } else if let Some(v) = s.strip_prefix("PT").and_then(|v| v.strip_suffix('M')) {
            v.parse::<i64>()
                .map_err(serde::de::Error::custom)?
                * 60
        } else {
            return Err(serde::de::Error::custom("unsupported xs:duration"));
        };

        Ok(Duration::seconds(seconds))
    }
}
