use serde::Deserialize;
use serde::de::{self, Deserializer};

// Custom deserialization function to convert a string to i32
pub fn string_to_i32<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<i32>().map_err(de::Error::custom)
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde::de::value::StrDeserializer;
    use serde::de::IntoDeserializer;

    #[test]
    fn test_string_to_i32_valid() {
        let deserializer: StrDeserializer<de::value::Error> = "42".into_deserializer();
        let result = string_to_i32(deserializer);
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_string_to_i32_invalid() {
        let deserializer: StrDeserializer<de::value::Error> = "invalid".into_deserializer();
        let result = string_to_i32(deserializer);
        assert!(result.is_err());
    }

    #[test]
    fn test_string_to_i32_empty() {
        let deserializer: StrDeserializer<de::value::Error> = "".into_deserializer();
        let result = string_to_i32(deserializer);
        assert!(result.is_err());
    }
}
