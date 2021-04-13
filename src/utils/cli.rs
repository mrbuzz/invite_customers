use anyhow::{Context, Result};

const DEFAULT_INVITE_KILOMETER_RANGE: f64 = 100.0_f64;
///
/// Utility method to get the correct kilometers range, if nothing is
/// specified defaults to 100 km 
///
pub fn get_kilometers_range(range: Option<&str>) -> Result<f64> {
  match range {
    Some(ref range) => {
      range.parse::<f64>()
        .with_context(|| format!("Unable to parse given range: {}", range))
    },
    None => Ok(DEFAULT_INVITE_KILOMETER_RANGE),
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn get_kilometers_range_errors_out_with_invalid_imput() -> Result<()> {
    let range = get_kilometers_range(Some("45h"));

    assert_eq!(range.unwrap_err().to_string(), "Unable to parse given range: 45h");
    Ok(())
  }

  #[test]
  fn get_kilometers_range_returns_default_if_no_range_has_been_specified() -> Result<()> {
    let range = get_kilometers_range(None)?;

    assert_eq!(range, DEFAULT_INVITE_KILOMETER_RANGE);
    Ok(())
  }

  #[test]
  fn get_kilometers_range_returns_the_specified_range() -> Result<()> {
    let range = get_kilometers_range(Some("45"))?;

    assert_eq!(range, 45.0_f64);
    Ok(())
  }

}
