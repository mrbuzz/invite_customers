use anyhow::{Context, Result};
use std::{fs::{File, read_to_string}, io::Stdout};
use ::std::io::{self, Write};
#[derive(Debug)]
pub enum OutputFile {
  File(File),
  Stdout(Stdout)
}
///
/// Trait implementation needed to make OutputFile as a generic 
/// writing interface. 
///
impl Write for OutputFile {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
      match self {
        OutputFile::File(f) => f.write(buf),
        OutputFile::Stdout(s ) =>  s.write(buf)
      }
    }

    fn flush(&mut self) -> io::Result<()> {
      match self {
        OutputFile::File(f) => f.flush(),
        OutputFile::Stdout(s ) =>  s.flush()
      }
    }
}
///
/// Utility method to get the desired output file, if nothing is
/// specified defaults to standard output.
///
pub fn get_output_file(output: Option<&str>) -> Result<OutputFile> {
  match output {
    Some(path) => Ok(OutputFile::File(File::create(path)?)),
    None => Ok(OutputFile::Stdout(io::stdout())),
  }
}

pub fn read_file_to_string<S>(path:  S) -> Result<String> 
where S: Into<String>
{
  read_to_string(path.into())
    .context(format!("Could not read from file"))
}

#[cfg(test)]
mod test {
  use super::*;
  use tempfile::NamedTempFile;

  #[test]
  fn get_output_file_creates_the_file_specified() -> Result<()> {
    let file_path_only = NamedTempFile::new()?;
    let path = file_path_only.into_temp_path();

    let output_file = get_output_file(Some(path.to_str().context("Invalid path")?))?;

    assert!(matches!(output_file, OutputFile::File(_)));
    Ok(())
  }
  #[test]
  fn get_output_file_replaces_the_file_if_it_already_exist() -> Result<()> {
    let file = NamedTempFile::new()?;
    let output_file = get_output_file(Some(file.path().to_str().context("Invalid path")?))?;

    assert!(matches!(output_file, OutputFile::File(_)));
    Ok(())
  }

  #[test]
  fn get_output_file_returns_stdout_when_no_file_is_specified() -> Result<()> {
    let output_file = get_output_file(None)?;

    assert!(matches!(output_file, OutputFile::Stdout(_)));
    Ok(())
  }

  #[test]
  fn get_output_file_errors_out_when_an_invalid_path_is_specified() -> Result<()> {
    let output_file = get_output_file(Some("/path/to/nowhere"));

    assert_eq!(output_file.unwrap_err().to_string(), "No such file or directory (os error 2)");
    Ok(())
  }
  
  #[test]
  fn read_file_to_string_errors_out_if_file_does_not_exist() {
    let file_contents = read_file_to_string("test_data/non_existing_file.txt");

    assert_eq!(file_contents.unwrap_err().to_string(), "Could not read from file");
  }

  #[test]
  fn read_file_to_string_reads_file_contents_into_a_string() -> Result<()> {
    let file_contents = read_file_to_string("test_data/valid_data.txt")?;

    assert_eq!(file_contents, "{\"latitude\": \"53.1229599\", \"user_id\": 6, \"name\": \"Theresa Enright\", \"longitude\": \"-6.2705202\"}\n{\"latitude\": \"53.1302756\", \"user_id\": 5, \"name\": \"Nora Dempsey\", \"longitude\": \"-6.2397222\"}");

    Ok(())
  }
}