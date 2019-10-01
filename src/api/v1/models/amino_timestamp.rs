use serde::de::Visitor;
use serde::export::fmt;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct AminoTimestamp(u64);

impl AminoTimestamp {
    pub fn from_current_time() -> Self {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        Self(since_the_epoch.as_millis() as u64)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl Serialize for AminoTimestamp {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.0)
    }
}

impl<'de> Deserialize<'de> for AminoTimestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u64(AminoTimestampVisitor)
    }
}

struct AminoTimestampVisitor;

impl<'de> Visitor<'de> for AminoTimestampVisitor {
    type Value = AminoTimestamp;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Expected a large positive number")
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(AminoTimestamp(value))
    }
}
