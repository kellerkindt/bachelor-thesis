use std::fs::File;
use std::io::Write;
use std::io::Error;

fn main() {
  if let Err(e) = schreibe_schluessel("private.key", "42") {
    println!("Fehler aufgetreten: {}", e);
  }
}

fn schreibe_schluessel(file: &str, content: &str) -> Result<(), Error>  {
  let mut file = File::create(file)?;
  write!(file, "{}", content)?;
  Ok(())
}
