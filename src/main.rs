mod utils;
mod customer;

use anyhow::Result;
use clap::{load_yaml, App};
use customer::invite_list::InviteList;
use utils::{cli::get_kilometers_range, files::{read_file_to_string, get_output_file}};
use std::io::Write;

fn main() -> Result<()> {
  let yaml = load_yaml!("cli.yml");
  let matches = App::from_yaml(yaml).get_matches();
  let path = matches.value_of("INPUT").unwrap();

  let mut output_file = 
    get_output_file(matches.value_of("output") )?;

  let range = 
    get_kilometers_range(matches.value_of("range"))?;

  let file_contents = 
    read_file_to_string(path)?;
  
  let invite_list = 
    InviteList::new()
      .with_data(file_contents)
      .with_range(range)
      .sorted_by_user_id()
      .build()?;

  for customer in invite_list {
    writeln!(output_file, "{}", customer)?;
  } 

  Ok(())
}