use std::sync::{Arc, Barrier, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::thread::{JoinHandle};
use std::time::{Duration};
use crate::util::data_struct::ordered_linked_list::PoppedResult;
use crate::util::worker::message_queue::{MessageQueue, Task, FLAG_TERMINATE, FLAG_BARRIER, has_flag};

struct Terminate {}
impl Task for Terminate {
    fn get_flag(&self) -> usize { FLAG_TERMINATE }
}


struct BarrierTask {
    barrier: Arc<Barrier>,
}
impl Task for BarrierTask {
    fn get_flag(&self) -> usize { FLAG_BARRIER }

    fn run(self: Box<Self>, _: &usize) {
        self.barrier.wait();
    }
}


#[allow(dead_code)]
struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<usize>>>, queue: Arc<Mutex<MessageQueue>>) -> Self {
        Worker {
            id,
            thread: Some(Self::build_thread(id.clone(), receiver, queue)),
        }
    }

    fn build_thread(id: usize, receiver: Arc<Mutex<Receiver<usize>>>, queue: Arc<Mutex<MessageQueue>>) -> JoinHandle<()> {
        thread::spawn(move || {
            loop {
                let result = { queue.lock().unwrap().pop() };
                match result {
                    PoppedResult::Empty => {
                        //println!("Worker#{}: Queue is empty, wait", id);
                        if let Ok(sig) = receiver.lock().unwrap().recv() {
                            //println!("Worker#{}: Empty and received signal: {}", id, sig);
                            if has_flag(sig, FLAG_TERMINATE) {
                                break;
                            }
                            continue;
                        } else {
                            //break;
                        }
                    },
                    PoppedResult::Unavailable(delay) => {
                        //println!("Worker#{}: Has task but not ready, wait for {}ms", id, delay);
                        if let Ok(_) = receiver.lock().unwrap().recv_timeout(Duration::from_millis(delay as u64)) {
                            //println!("Worker#{}: Unavailable and received signal: {}", id, sig);
                            continue;
                        } else {
                            //break;
                        }
                    },
                    PoppedResult::Success(task) => {
                        let flag = task.get_flag();
                        //println!("Worker#{}: Got a task (flag: {})", id, flag);
                        task.run(&id);
                        match flag {
                            n if has_flag(n, FLAG_TERMINATE) => {
                                break;
                            },
                            _ => (),
                        }
                    },
                }
            }
        })
    }
}

#[allow(dead_code)]
pub struct WorkerManager {
    name: String,
    sender: Sender<usize>,      // Signal sender
    workers: Vec<Worker>,
    size: usize,
    queue: Arc<Mutex<MessageQueue>>,
}

impl WorkerManager {
    pub fn new(name: String, size: usize) -> Self {
        assert!(size > 0);

        let queue = Arc::new(Mutex::new(MessageQueue::new()));

        let (sender, receiver) = channel::<usize>();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver), queue.clone()))
        }

        WorkerManager {
            name,
            sender,
            workers,
            size,
            queue
        }
    }

    pub fn schedule<T: Task + Send + 'static>(&mut self, task: T) {
        self.schedule_delayed(task, 0);
    }

    pub fn schedule_delayed<T: Task + Send + 'static>(&mut self, task: T, delay: u128) {
        self.queue.lock().unwrap().schedule_delayed(task, delay);
        let _ = self.sender.send(0);
    }

    #[allow(dead_code)]
    pub fn schedule_end<T: Task + Send + 'static>(&mut self, task: T) {
        self.schedule_end_flag(task, 0);
    }

    fn schedule_end_flag<T: Task + Send + 'static>(&mut self, task: T, flag: usize) {
        self.queue.lock().unwrap().schedule_to_end(task);
        let _ = self.sender.send(flag);
    }

    pub fn place_barrier(&mut self) {
        let barrier = Arc::new(Barrier::new(self.size + 1));

        for _ in 0..self.size {
            let c = Arc::clone(&barrier);
            let task = BarrierTask { barrier: c };
            self.schedule_end_flag(task, FLAG_BARRIER);
        }
        barrier.wait();
        //println!("WorkerManager({}): Rendezvoused", self.name)
    }

    #[allow(dead_code)]
    pub fn queue_len(&self) -> usize {
        self.queue.lock().unwrap().len()
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn shutdown(&mut self) {
        //println!("WorkerManager({}): Sending terminate message to all workers.", self.name);
        for _ in 0..self.size {
            self.schedule_end_flag(Terminate {}, FLAG_TERMINATE);
        }

        //println!("WorkerManager({}): Shutting down all workers.", self.name);

        for worker in &mut self.workers {
            //println!("WorkerManager({}): Shutting down worker {}", self.name, worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
                //println!("WorkerManager({}): Terminated Worker#{}", self.name, worker.id);
            }
        }
    }
}

impl Drop for WorkerManager {
    fn drop(&mut self) {
        self.shutdown()
    }
}


#[cfg(test)]
mod test {
    use super::WorkerManager;

    #[test]
    fn it_works() {
        let mut wm = WorkerManager::new("123".to_string(), 3);
        schedule_num(&mut wm, 10);
        schedule_num(&mut wm, 5);
        schedule_num(&mut wm, 0);
        schedule_num(&mut wm, 8);
        schedule_num(&mut wm, 0);
        schedule_num(&mut wm, 1000);
        schedule_num(&mut wm, 0);
        schedule_num(&mut wm, 3);
        //println!("size: {}", { wm.queue_len() });
        wm.place_barrier();
        //println!("After barrier");
        drop(wm);
    }

    fn schedule_num(wm: &mut WorkerManager, num: u128) {
        wm.schedule_delayed(move || {
            //println!("===> {}", num)
        }, num);
    }

    #[test]
    fn test_barrier() {
        let mut wm = WorkerManager::new("123".to_string(), 3);
        schedule_num(&mut wm, 10);
        schedule_num(&mut wm, 0);
        schedule_num(&mut wm, 0);
        //println!("before barrier");
        wm.place_barrier();
        //println!("after barrier");
        schedule_num(&mut wm, 3);
        drop(wm);
    }
}
