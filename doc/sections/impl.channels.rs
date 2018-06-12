use std::thread;
use std::sync::mpsc;

fn main() {
  let (sender, receiver) = mpsc::channel();

  let thread = thread::spawn(move|| {
    println!("Empfangen: {}", receiver.recv().unwrap());
  });

  sender.send("Hallo, Kanal!").unwrap();
  thread.join().unwrap();
}
