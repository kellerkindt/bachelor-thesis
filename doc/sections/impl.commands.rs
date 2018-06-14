use std::thread;
use std::sync::mpsc;

enum Command {
  Say(String),
}

impl Command {
  fn execute(self) {
    match self {
      Command::Say(text) => println!("Say: {}", text),
    }
  }
}

fn main() {
  let (sender, receiver) = mpsc::channel();

  let thread = thread::spawn(move || {
    let command: Command = receiver.recv().unwrap();
    command.execute();
  });

  sender.send(Command::Say("Hallo, Kanal!".into())).unwrap();
  thread.join().unwrap();
}
