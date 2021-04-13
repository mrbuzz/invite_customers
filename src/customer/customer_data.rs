use anyhow::Result;
use serde::{de, Deserialize, Deserializer};
///
/// CustomerData represents raw customer data that comes from the imput file
/// the only transformation that is applied at this stage is the conversion
/// from String to f64 using the 'de_from_str_to_f64' method
///
#[derive(Debug, Deserialize, PartialEq, PartialOrd)]
pub struct CustomerData {
    #[serde(deserialize_with = "de_from_str_to_f64")]
    pub latitude: f64,
    #[serde(deserialize_with = "de_from_str_to_f64")]
    pub longitude: f64,
    pub user_id: i64,
    pub name: String 
}
///
/// This deserializer function is used by the serde crate to 
/// perform the conversion between String and f64 when the data
/// is deserialized
///
fn de_from_str_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    s.parse::<f64>().map_err(de::Error::custom)
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn json_deserialization_works_correctly_with_valid_data() -> Result<()> {
    let json_data = r#" {"latitude": "52.833502", "user_id": 25, "name": "David Behan", "longitude": "-8.522366"} "#;
    let deserialized: CustomerData = serde_json::from_str(json_data)?;
    
    assert_eq!(deserialized.user_id, 25);
    assert_eq!(deserialized.name, "David Behan");
    assert_eq!(deserialized.latitude,  52.833502_f64);
    assert_eq!(deserialized.longitude, -8.522366_f64);

    Ok(())
  }

  #[test]
  fn returns_err_if_given_invalid_json() -> Result<()> {
    let json_data = r#" {"latitude" "52.833502", "user_id": 25, "name": "David Behan", "longitude": "-8.522366"} "#;
    let deserialized: std::result::Result<CustomerData, serde_json::Error> = serde_json::from_str(json_data);
    
    assert_eq!(deserialized.unwrap_err().to_string(), "expected `:` at line 1 column 14");

    Ok(())
  }
}