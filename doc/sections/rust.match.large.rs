fn main() {
    let value : Option<&str> = Some("text");
    match value {
        Some("test") => println!("Nur ein Test"),
        Some(value) => println!("Wert ist: {}", value),
        None => println!("Kein Wert"),
    };
}
