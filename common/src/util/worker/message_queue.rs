use std::time::{SystemTime, UNIX_EPOCH};
use crate::util::data_struct::ordered_linked_list::{OrderedLinkedList, PoppedResult};

pub const FLAG_DEFAULT: usize = 0b00000000;
pub const FLAG_TERMINATE: usize = 0b00000001;
pub const FLAG_BARRIER: usize = 0b00000010;

pub fn has_flag(flag: usize, target: usize) -> bool {
    flag & target == target
}

pub trait Task {
    fn get_flag(&self) -> usize {
        FLAG_DEFAULT
    }

    #[allow(unused_variables)]
    fn run(self: Box<Self>, id: &usize) {}
}

impl<F> Task for F where F: FnOnce() {
    fn run(self: Box<Self>, _: &usize) {
        (*self)();
    }
}

pub type RawTask = dyn Task + Send + 'static;
pub type Message = Box<RawTask>;

pub struct MessageQueue {
    list: OrderedLinkedList<Message, u128>
}

impl MessageQueue {
    pub fn new() -> Self {
        MessageQueue {
            list: OrderedLinkedList::new()
        }
    }

    #[allow(dead_code)]
    pub fn schedule<T>(&mut self, handler: T)
        where T: Task + Send + 'static
    {
        self.schedule_delayed(handler, 0);
    }

    pub fn schedule_delayed<T>(&mut self, handler: T, delay_ms: u128)
        where T: Task + Send + 'static
    {
        let crt_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went back!").as_millis();
        self.list.push_by(Box::new(handler), crt_time + delay_ms);
    }

    pub fn schedule_to_end<T>(&mut self, handler: T)
        where T: Task + Send + 'static
    {
        self.list.push_end(Box::new(handler));
    }

    pub fn pop(&mut self) -> PoppedResult<Message, u128> {
        let crt_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went back!").as_millis();
        let result = self.list.pop_by(crt_time);
        if let PoppedResult::Unavailable(target) = result {
            PoppedResult::Unavailable(target - crt_time)
        } else {
            result
        }
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }
}

unsafe impl Sync for MessageQueue {}
unsafe impl Send for MessageQueue {}