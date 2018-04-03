use std::sync::Mutex;

fn main() {
    let mutex = Mutex::new(0);
    let lock  = mutex.lock();

    if let Ok(mut lock) = lock {
        *lock += 1;
        println!("Wert: {}", lock); // "Wert: 1"
    }
}
