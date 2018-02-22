use std::fs::File;
use std::io::Write;

fn main() {
    match File::open("private.key") {
        Err(e) => println!("Fehler aufgetreten: {}", e),
        Ok(mut file) => {
            let _ = write!(file, "42");
        }
    }
}
