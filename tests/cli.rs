use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use anyhow::{Context, Result};
use tempfile::NamedTempFile;

#[test]
fn when_file_doesnt_exist() -> Result<()> {
    let mut cmd = Command::cargo_bin("invite_customers")?;

    cmd.arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn when_file_exists() -> Result<()> {
    let mut cmd = Command::cargo_bin("invite_customers")?;

    cmd.arg("test_data/valid_data.txt");
    cmd.assert()
       .success()
       .stdout(predicate::str::contains("UserID: 5, Name: Nora Dempsey, Distance from Dublin HQ: 23 Km\nUserID: 6, Name: Theresa Enright, Distance from Dublin HQ: 24 Km\n"));

    Ok(())
}

#[test]
fn when_when_no_range_is_specified() -> Result<()> {
    let mut cmd = Command::cargo_bin("invite_customers")?;
    let stdout = 
r#"UserID: 4, Name: Ian Kehoe, Distance from Dublin HQ: 11 Km
UserID: 5, Name: Nora Dempsey, Distance from Dublin HQ: 23 Km
UserID: 6, Name: Theresa Enright, Distance from Dublin HQ: 24 Km
UserID: 8, Name: Eoin Ahearn, Distance from Dublin HQ: 84 Km
UserID: 11, Name: Richard Finnegan, Distance from Dublin HQ: 38 Km
UserID: 12, Name: Christina McArdle, Distance from Dublin HQ: 42 Km
UserID: 13, Name: Olive Ahearn, Distance from Dublin HQ: 62 Km
UserID: 15, Name: Michael Ahearn, Distance from Dublin HQ: 44 Km
UserID: 17, Name: Patricia Cahill, Distance from Dublin HQ: 96 Km
UserID: 23, Name: Eoin Gallagher, Distance from Dublin HQ: 83 Km
UserID: 24, Name: Rose Enright, Distance from Dublin HQ: 89 Km
UserID: 26, Name: Stephen McArdle, Distance from Dublin HQ: 99 Km
UserID: 29, Name: Oliver Ahearn, Distance from Dublin HQ: 72 Km
UserID: 30, Name: Nick Enright, Distance from Dublin HQ: 83 Km
UserID: 31, Name: Alan Behan, Distance from Dublin HQ: 44 Km
UserID: 39, Name: Lisa Ahearn, Distance from Dublin HQ: 38 Km
"#;

    cmd.arg("test_data/complete_data.txt");
    cmd.assert()
      .success()
      .stdout(predicate::str::contains(stdout));

    Ok(())
}

#[test]
fn when_when_a_range_is_specified() -> Result<()> {
    let mut cmd = Command::cargo_bin("invite_customers")?;
    let stdout = 
r#"UserID: 4, Name: Ian Kehoe, Distance from Dublin HQ: 11 Km
UserID: 5, Name: Nora Dempsey, Distance from Dublin HQ: 23 Km
UserID: 6, Name: Theresa Enright, Distance from Dublin HQ: 24 Km
"#;

    cmd.arg("test_data/complete_data.txt")
       .arg("--range")
       .arg("25");
    cmd.assert()
      .success()
      .stdout(predicate::str::contains(stdout));

    Ok(())
}

#[test]
fn when_invalid_range_is_specified() -> Result<()> {
    let mut cmd = Command::cargo_bin("invite_customers")?;

    cmd.arg("test_data/complete_data.txt")
    .arg("--range")
    .arg("25r");
    cmd.assert()
      .failure()
      .stderr(predicate::str::contains("Unable to parse given range: 25r"));

    Ok(())
}

#[test]
fn when_an_input_file_with_invalid_data_is_specified() -> Result<()> {
    let mut cmd = Command::cargo_bin("invite_customers")?;

    cmd.arg("test_data/invalid_data.txt");
    cmd.assert()
      .failure()
      .stderr(predicate::str::contains("Invalid JSON string"));

    Ok(())
}

#[test]
fn when_an_output_file_is_specified() -> Result<()> {
  let temp_file = NamedTempFile::new()?;
  let mut cmd = Command::cargo_bin("invite_customers")?;

  cmd.arg("test_data/valid_data.txt")
    .arg("--output")
    .arg(temp_file.path().to_str().context("Invalid string")?);

  cmd.assert()
    .success()
    .stdout(predicate::str::contains(""));

  Ok(())
}

#[test]
fn when_an_invalid_output_file_is_specified() -> Result<()> {
  let mut cmd = Command::cargo_bin("invite_customers")?;

  cmd.arg("test_data/valid_data.txt")
    .arg("--output")
    .arg("invalid/path");

  cmd.assert()
    .failure()
    .stderr(predicate::str::contains("No such file or directory (os error 2)"));

  Ok(())
}