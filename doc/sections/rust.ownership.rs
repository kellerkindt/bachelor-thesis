fn main() {
    let mut a = Box::new(1.0_f32); // Eigentümer der neuen
                                   // Heap-Variable ist a

    {
        let b = &a; // a wird an b mit Lesezugriff verliehen
        let c = &a; // a wird an c mit Lesezugriff verliehen

        println!("a: {}", a); // "a: 1"
        println!("b: {}", b); // "b: 1"
        println!("c: {}", c); // "c: 1"

        // let d = &mut a; // Nicht erlaubt: Es existieren
                           // verliehene Lesezugriffe

        // *a = 7_f32; // Nicht erlaubt: Es existieren
                       // verliehene Lesezugriffe

    } // Ende von b und c, a nicht mehr verliehen

    {
        let e = &mut a; // Leihe a mit Schreiberlaubnis
        **e = 9_f32;    // Setze Inhalt von a

        // println!("a: {}", a); // Nicht erlaubt: exklusiver
                                 // Zugriff an e verliehen

        println!("e: {}", e); // "e: 9"

    } // Ende von e, a nicht mehr verliehen

    println!("a: {}", a); // "a: 9"
    let f = a; // Neuer Eigentümer der Heap-Variable ist f
    // *a = 12.5_f32; // Nicht erlaubt: Nicht mehr Eigentümer
    // *f = 12.5_f32; // Nicht erlaubt: f nicht änderlich
    println!("f: {}", f); // "f: 9"
}
