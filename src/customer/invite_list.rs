use anyhow::Result;
use super::customer::Customer;
///
/// Implementation of the Build design pattern. Builds a list 
/// of customers as a std::Vec<Customer> for the given parameters.
///
pub struct InviteList {
    data: String,
    range: f64,
    sort_by_user_id: bool
}

impl InviteList {
  pub fn new() -> InviteList {
    InviteList {
      data: String::from(""),
      range: f64::INFINITY,
      sort_by_user_id: false
    }
  }

  pub fn with_data<S>(mut self, data: S) -> InviteList
  where S: Into<String>
  {
    self.data = data.into();
    self
  }

  pub fn with_range(mut self, range: f64) -> InviteList {
    self.range = range;
    self
  }

  pub fn sorted_by_user_id(mut self) -> InviteList {
    self.sort_by_user_id = true;
    self
  }

  pub fn build(self) -> Result<Vec<Customer>> {
    match Customer::list_from_json(&self.data) {
      Ok(mut list) => {
        list.retain(|customer| customer.distance_from_dublin <= self.range);
        if self.sort_by_user_id {
          list.sort_by_key(|customer| customer.user_id);
        }
        Ok(list)
      },
      Err(err) => Err(err)
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use predicates::prelude::*;

  #[test]
  fn errors_out_with_invalid_data() {
    let json_data = r#" {"latitude"![e] "52.833502", "user_id": 25, "name": "David Behan", "longitude": "-8.522366"}
    {"latitude": "54.080556", "user_id": 23, "name": "Eoin Gallagher", "longitude": "-6.361944"} "#;
    let invite_list = InviteList::new().with_data(json_data).build();
    
    assert_eq!(invite_list.unwrap_err().to_string(), "Invalid JSON string");
  }
  
  #[test]
  fn builds_an_invite_list() -> Result<()> {
    let json_data = r#" {"latitude": "52.833502", "user_id": 25, "name": "David Behan", "longitude": "-8.522366"}
                          {"latitude": "54.080556", "user_id": 23, "name": "Eoin Gallagher", "longitude": "-6.361944"} "#;
    let invite_list = InviteList::new().with_data(json_data).build()?;
    assert_eq!(invite_list.len(), 2);

    Ok(())
  }

  #[test]
  fn builds_an_invite_list_sorted_by_user_id() -> Result<()> {
    let json_data =  
    r#" {"latitude": "52.833502", "user_id": 25, "name": "David Behan", "longitude": "-8.522366"}
        {"latitude": "54.080556", "user_id": 23, "name": "Eoin Gallagher", "longitude": "-6.361944"} "#;
    let invite_list = InviteList::new().with_data(json_data).sorted_by_user_id().build()?;
    
    assert_eq!(invite_list.len(), 2);
    assert_eq!(invite_list[0].user_id, 23);
    
    Ok(())
  }

  #[test]
  fn builds_an_invite_list_within_given_kilometers() -> Result<()> {
    let json_data =  
    r#" {"latitude": "53.2451022",  "user_id": 4,   "name": "Ian Kehoe",        "longitude": "-6.238335"}
        {"latitude": "53.1302756",  "user_id": 5,   "name": "Nora Dempsey",     "longitude": "-6.2397222"}
        {"latitude": "53.1229599",  "user_id": 6,   "name": "Theresa Enright",  "longitude": "-6.2705202"}
        {"latitude": "54.374208",   "user_id": 22,  "name": "Charlie McArdle",  "longitude": "-8.371639"}
        {"latitude": "53.74452",    "user_id": 29,  "name": "Oliver Ahearn",    "longitude": "-7.11167"}
        {"latitude": "53.761389",   "user_id": 30,  "name": "Nick Enright",     "longitude": "-7.2875"}
        {"latitude": "54.080556",   "user_id": 23,  "name": "Eoin Gallagher",   "longitude": "-6.361944"} "#;
    let invite_list = InviteList::new().with_data(json_data).with_range(25.0_f64).build()?;
    

    let contains_user_id = 
      predicate::in_iter(invite_list.iter().map(|customer| customer.user_id));

    assert_eq!(true, contains_user_id.eval(&4));
    assert_eq!(true, contains_user_id.eval(&5));
    assert_eq!(true, contains_user_id.eval(&6));
    assert_eq!(false, contains_user_id.eval(&22));
    assert_eq!(false, contains_user_id.eval(&23));
    assert_eq!(false, contains_user_id.eval(&29));
    assert_eq!(false, contains_user_id.eval(&30));

    Ok(())
  }
}