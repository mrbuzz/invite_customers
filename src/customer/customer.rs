use anyhow::{Context, Result};
use super::customer_data::CustomerData;
use std::fmt;
use serde::Deserialize;
use super::super::utils::math::compute_distance_from_dublin_hq;
///
/// Customer represents the data format used internally,
/// conversion from CustomerData happens under the hood 
/// thanks to the implementation of the From<CustomerData>
/// trait.
///
#[derive(Debug, Deserialize)]
#[serde(from = "CustomerData")]
pub struct Customer {
  pub user_id: i64,
  pub name: String,
  pub distance_from_dublin: f64
}
///
/// Deserializing JSON directly to Customer works thanks 
/// to this trait. 
///
impl From<CustomerData> for Customer {
  fn from(data: CustomerData) -> Self {
    Self { 
      user_id: data.user_id, 
      name: data.name, 
      distance_from_dublin: compute_distance_from_dublin_hq(data.latitude, data.longitude)
    }
  }
}

impl fmt::Display for Customer {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "UserID: {}, Name: {}, Distance from Dublin HQ: {} Km", self.user_id, self.name, self.distance_from_dublin)
  }
}

impl Customer {
  pub fn from_json<S>(json_data: S) -> Result<Self>
  where S: Into<String>
  {
    serde_json::from_str(json_data.into().as_str())
      .with_context(|| format!("Invalid JSON string"))
  }

  pub fn list_from_json<S>(json_data: S) -> Result<Vec<Self>>
  where S: Into<String>
  {
    json_data.into().split('\n')
      .map( |customer_data| 
        Customer::from_json(customer_data)
      ).collect()
  }
}


#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn from_json_works_correctly_with_valid_data() -> Result<()> {
    let json_data = r#" {"latitude": "52.833502", "user_id": 25, "name": "David Behan", "longitude": "-8.522366"} "#;
    let deserialized = Customer::from_json(json_data)?;
    
    assert_eq!(deserialized.user_id, 25);
    assert_eq!(deserialized.name, "David Behan");
    assert_eq!(deserialized.distance_from_dublin, 161.0_f64);

    Ok(())
  }

  #[test]
  fn from_json_errors_out_with_invalid_data() {
    let json_data = r#" {"latitude": "52.833502", "user_id" 25, "name": "David Behan", "longitude": "-8.522366"} "#;
    let deserialized = Customer::from_json(json_data);
    
    assert_eq!(deserialized.unwrap_err().to_string(), "Invalid JSON string");
  }

  #[test]
  fn list_from_json_works_correctly_with_valid_data() -> Result<()> {
    let json_data = r#" {"latitude": "52.833502", "user_id": 25, "name": "David Behan", "longitude": "-8.522366"}
                             {"latitude": "54.080556", "user_id": 23, "name": "Eoin Gallagher", "longitude": "-6.361944"} "#;
    let deserialized = Customer::list_from_json(json_data)?;
    
    assert_eq!(deserialized.len(), 2);

    Ok(())
  }

  #[test]
  fn list_from_json_errors_out_with_invalid_data() {
    let json_data = r#" {"latitude": "52.833502", "user_id": 25, "name": "David Behan", "longitude": "-8.522366"}\n{"latitude": "54.080556", "user_id": 23, "name": "Eoin Gallagher", "longitude": "-6.361944"}"#;
    let deserialized = Customer::list_from_json(json_data);
    
    assert_eq!(deserialized.unwrap_err().to_string(), "Invalid JSON string");
  }
}