use std::sync::Mutex;

fn main() {
    let mutex = Mutex::new(0);
    let lock  = mutex.lock();

    if let Ok(mut counter) = lock {
        *counter += 1;
        println!("Zähler: {}", counter); // "Zähler: 1"
    }
}
