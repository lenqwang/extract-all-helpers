use std::{fs::File, io::Read};

pub(crate) fn read_file_content(path: &str) -> std::io::Result<String> {
  let mut file = File::open(path).unwrap();
  let mut contents = String::new();

  file.read_to_string(&mut contents)?;

  Ok(contents.to_string())
}