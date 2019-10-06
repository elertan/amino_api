use serde::de::{SeqAccess, Visitor};
use serde::export::fmt;
use serde::{ser::SerializeSeq, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct MediaListItem {
    pub some_number: serde_json::Value,
    pub source_url: String,
    pub something_else: Option<serde_json::Value>,
}

impl Serialize for MediaListItem {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(3))?;
        seq.serialize_element(&self.some_number)?;
        seq.serialize_element(&self.source_url)?;
        seq.serialize_element(&self.something_else)?;
        seq.end()
    }
}

impl<'de> Deserialize<'de> for MediaListItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(MediaListItemVisitor)
    }
}

struct MediaListItemVisitor;

impl<'de> Visitor<'de> for MediaListItemVisitor {
    type Value = MediaListItem;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Expected sequence of 3 values")
    }

    fn visit_seq<E>(self, mut visitor: E) -> Result<Self::Value, E::Error>
    where
        E: SeqAccess<'de>,
    {
        let mut vec: Vec<serde_json::Value> = Vec::with_capacity(3);

        while let Some(elem) = visitor.next_element()? {
            vec.push(elem);
        }

        let mut iter = vec.into_iter();

        let media_list_item = MediaListItem {
            some_number: iter.next().unwrap(),
            source_url: iter.next().unwrap().to_string(),
            something_else: iter.next(),
        };

        Ok(media_list_item)
    }
}
