use std::io::stdin;

fn main() {
    let mut eingabe = String::new();
    while let Ok(_) = stdin().read_line(&mut eingabe) {
        println!("Eingabe: {}", eingabe.trim());
        eingabe.clear();
    }
}
