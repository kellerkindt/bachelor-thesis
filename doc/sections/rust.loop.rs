const VERBOTENES_ZEICHEN : &str = "#";

fn main() {
    let name = loop {
        let name = ...; // Lese Zeile von stdin
        if name.contains(VERBOTENES_ZEICHEN) {
            println!("Versuchs nochmal");
        } else {
            break name;
        }
    };

    println!("Gültiger Name: {}", name);
}
