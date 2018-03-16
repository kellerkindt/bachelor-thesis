struct Computer<'a> {
    model: &'a str,
}

fn erstelle_computer<'a>(model: &'a str) -> Computer<'a> {
    Computer { model }
}

fn klone_computer<'a>(original: &'a Computer) -> Computer<'a> {
    Computer { model: original.model }
}

fn loesche_computer(_: Computer) {
    // löschen durch Konsum
}

fn main() {
   // Zeichenkette auf dem Heap
   let model = String::from("Lenovo");

   let c1 = erstelle_computer(&model);
   let c2 = klone_computer(&c1);
   println!("Model Nr1={}, Nr2={}\n", c1.model, c2.model);

   loesche_computer(c1);
   println!("Model Nr2: {}", c2.model);
}
