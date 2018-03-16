fn main() { // neuer Scope
    let mut a = Box::new(5); // 5 kommt auf den Heap
    { // neuer Scope
        let b = Box::new(10); // 10 kommt auch auf den Heap
        *a += *b; // a ist nun 15 
    } // Lebenszeit von b zu Ende, Speicher wird freigeben
    println!("a: {}", a); // Ausgabe: "a: 15"
} // Lebenszeit von a zu Ende, Speicher wird freigegeben
