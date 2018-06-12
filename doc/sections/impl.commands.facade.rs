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

trait EndPoint {
  fn say(&self, text: &str);
}

impl EndPoint for mpsc::Sender<Command> {
  fn say(&self, text: &str) {
    self.send(Command::Say(text.into())).unwrap();
  }
}

fn main() {
  let (sender, receiver) = mpsc::channel();
  let endpoint = &sender as &EndPoint;

  let thread = thread::spawn(move|| {
    let command: Command = receiver.recv().unwrap();
    command.execute();
  });

  endpoint.say("Hallo, Fassade!");
  thread.join().unwrap();
}
