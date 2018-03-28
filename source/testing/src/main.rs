
#[macro_use]
extern crate log;
extern crate log4rs;


extern crate tokio;
extern crate futures;

use std::thread;

use tokio::prelude::*;
use tokio::reactor::Reactor;
use tokio::net::TcpListener;

use futures::Sink;
use futures::Async;
use futures::Future;
use futures::stream::ForEach;
use futures::sync::mpsc::channel;
use futures::sync::mpsc::Sender;
use futures::sync::mpsc::Receiver;
use futures::sync::mpsc::SendError;

fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    info!(target: "blubber", "info!");

    println!("Hello, world!");


    let (counter_tx, counter_rx) = channel(32);

    {
        let tx = counter_tx.clone();
        thread::spawn(move|| {
            tx
                .send(CounterCommand::Increase)
                .and_then(|tx| tx.send(CounterCommand::Increase))
                .and_then(|tx| {
                    thread::sleep_ms(5000);
                    tx.send(CounterCommand::Increase)
                })
                .then(|result| {
                    println!("Result sending: {:?}", result);
                    match result {
                        Err(_) => Err(()),
                        Ok(_) => Ok(())
                    }
                })
                .wait();
            println!(":: after run");
        });
    }

    thread::sleep_ms(100);
    /*{
        let sender = new_rc(tx.clone());
        let sender_result = sender.send(String::from("lolz")).wait();
        println!("Sending to rc: {:?}", sender_result);
    }*/

    let (tcp_server_tx, tcp_server_rx) = channel(16);
    let (algorithm_tx, algorithm_rx) = channel(16);

    let platform = Platform {
        tcp_server: tcp_server_tx,
        algorithm:  algorithm_tx,
        counter: counter_tx,
    };



    thread::sleep_ms(100);
    {
        let platform = platform.clone();
        thread::spawn(move || {
            thread::sleep_ms(100);
            let _ = platform.tcp_server.clone().send(String::from("String")).wait();
            let _ = platform.tcp_server.clone().send(String::from("Another")).wait();
            let _ = platform.counter.clone().send(CounterCommand::Increase).wait();

            thread::sleep_ms(2000);

            let _ = platform.tcp_server.clone().send("close".to_string()).wait();
            let _ = platform.algorithm .clone().send("close".to_string()).wait();
            let _ = platform.counter   .clone().send(CounterCommand::Close).wait();

        });
    }

    let mut runtime = tokio::runtime::Runtime::new().expect("Failed to start runtime");
    runtime.spawn(create_message_counter(platform.clone(), counter_rx));
    runtime.spawn(create_algorithm(platform.clone(), algorithm_rx));
    runtime.spawn(create_tcp_server(platform.clone(), tcp_server_rx));

    drop(platform);
    runtime.shutdown_on_idle().wait().unwrap();

    warn!("Stopping");
}

#[derive(Clone)]
struct Platform {
    algorithm:  Sender<String>,
    tcp_server: Sender<String>,
    counter:    Sender<CounterCommand>,
}

fn create_algorithm(mut platform: Platform, receiver: Receiver<String>) -> impl Future<Item=(),Error=()> {
    println!("Starting algorithm");
    thread::sleep_ms(10);
    receiver.for_each(move |msg: String| {
        println!("Algorithm received: {}", msg);
        let _ = platform.counter.try_send(CounterCommand::Increase);
        if msg.eq("close") {
            println!(":: algorithm is closing");
            return Err(());
        }
        Ok(())
    })
}

fn create_tcp_server(mut platform: Platform, receiver: Receiver<String>) -> impl Future<Item=(),Error=()> {
    println!("Starting tcp server");
    receiver.for_each(move |msg: String| {
        println!("tcp server received: {}", msg);
        let _ = platform.algorithm.try_send(format!("tcp {}", msg));
        thread::sleep_ms(10);
        if msg.eq("close") {
            println!(":: tcp server is closing");
            return Err(());
        }
        Ok(())
    })
}

fn create_message_counter(platform: Platform, receiver: Receiver<CounterCommand>) -> impl Future<Item=(),Error=()> {
    adapt_command_processor(MessageCounter::new(), receiver)
}

fn adapt_command_processor<T, C: CommandProcessor<T>>(mut processor: C, receiver: Receiver<T>) -> impl Future<Item=(),Error=()> {
    receiver.for_each(move |command| processor.process(command))
}


pub struct MessageCounter {
    active: bool,
    counter: u32,
    listeners: Vec<Sender<String>>
}

#[derive(Debug)]
enum CounterCommand {
    Increase,
    Close
}

impl MessageCounter {
    fn new() -> MessageCounter {
        MessageCounter {
            active: true,
            counter: 0,
            listeners: Vec::new(),
        }
    }

    fn increase_counter(&mut self) {
        self.counter += 1;
        trace!("Counter is now {}", self.counter);
    }

    fn close(&mut self) {
        self.active = false;
        trace!("Counter is no longer active");
    }
}

impl CommandProcessor<CounterCommand> for MessageCounter {
    fn process_unchecked(&mut self, command: CounterCommand) -> Result<(), ()> {
        match command {
            CounterCommand::Increase => self.increase_counter(),
            CounterCommand::Close => {
                self.close();
                return Err(());
            }
        };
        Ok(())
    }

    fn is_active(&self) -> bool {
        self.active
    }
}

trait CommandProcessor<T> {
    fn process_unchecked(&mut self, command: T) -> Result<(), ()>;

    fn process(&mut self, command: T) -> Result<(), ()> {
        if self.is_active() {
            self.process_unchecked(command)
        } else {
            Err(())
        }
    }

    fn is_active(&self) -> bool;
}


/// Spawns the `Future` on its first call of `poll`. Returns `Ok(Async::Ready(()))`
/// on the first call and `Err(())` on every call afterwards
struct DeferredSpawn<T: Future<Item=(),Error=()> + Send + 'static> {
    once: Option<T>
}

impl<T: Future<Item=(),Error=()> + Send + 'static> DeferredSpawn<T> {
    pub fn new(once: T) -> DeferredSpawn<T> {
        DeferredSpawn {
            once: Some(once),
        }
    }
}

impl<T: Future<Item=(),Error=()> + Send + 'static> Future for DeferredSpawn<T> {
    type Item = ();
    type Error = ();
    fn poll(&mut self) -> Result<Async<()>, ()> {
        match self.once.take() {
            None => Err(()),
            Some(once) => {
                tokio::spawn(once);
                Ok(Async::Ready(()))
            }
        }
    }
}