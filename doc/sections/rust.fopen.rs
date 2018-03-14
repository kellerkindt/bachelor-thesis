use std::fs::File;
use std::io::Write;

fn main() {
  match File::create("private.key") {
    Err(e) => println!("Datei nicht erstellbar: {}", e),
    Ok(mut file) => {
      if let Err(e) = write!(file, "42") {
        println!("Konnte nicht in Datei schreiben: {}", e);
      }
    }
  }
}
