use std::{fs::File, io::{self, Read}, path::Path};

pub fn read_all(path: &Path) -> Result<String, io::Error> {
  let mut file = File::open(path)?;
  let mut str = String::new();
  file.read_to_string(&mut str)?;
  Ok(str)
}


