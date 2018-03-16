fn main() {
    let start = std::time::Instant::now();
    while start.elapsed().as_secs() < 1 {
        println!("Zeit noch nicht um");
    }
}
