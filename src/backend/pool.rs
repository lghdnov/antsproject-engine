use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use crossbeam::channel;
use crossbeam::channel::{bounded, Sender};
use rand::random;
use serde::{Deserialize, Serialize};
use crate::backend::task_processor::TaskProcessor;

#[derive(Deserialize, Serialize, Clone)]
pub struct Bot{
    pub name: String,
    pub code: String,
}
pub struct GameTask{
    pub bots: Vec<Bot>,
}

pub struct GamePool{
    sender: Sender<GameTask>,
    threads: Vec<thread::JoinHandle<()>>,
}

impl GamePool{
    pub fn new(num_threads: usize) -> Self {
        let (sender, receiver) = bounded(num_threads);
        let receiver = Arc::new(Mutex::new(receiver));

        let mut threads = Vec::with_capacity(num_threads);
        for _ in 0..num_threads {
            let receiver = Arc::clone(&receiver);
            let handle = thread::spawn(move || {
                worker_thread(receiver);
            });
            threads.push(handle);
        }

        GamePool { sender, threads }
    }

    pub fn add_task(&self, task: GameTask) {
        self.sender.send(task).expect("Failed to send task to worker threads");
    }
}

fn worker_thread(receiver: Arc<Mutex<channel::Receiver<GameTask>>>) {
    loop {
        let task = match receiver.lock().unwrap().recv() {
            Ok(task) => task,
            Err(_) => break,
        };

        process_task(task);
    }
}

fn process_task(task: GameTask) {

    let mut processor = TaskProcessor::new(task);

    match processor.initialize_bots() {
        Ok(()) => {
            println!("Initializing successfully completed");
        }
        Err(err) =>{
            println!("Error occurred while staring game: {err}");
            return;
        }
    }
    match processor.process() {
        Ok(()) => {}
        Err(err) => {
            println!("Error occurred in game loop: {err}");
        }
    }

}