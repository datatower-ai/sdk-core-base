use std::ptr::NonNull;

type Link<T, O> = Option<NonNull<Node<T, O>>>;

struct Node<T, O: Ord> {
    elem: T,        // the handler of this message
    order: O,             // when to handle this message
    next: Link<T, O>,
}

pub enum PoppedResult<T, O> {
    Empty,
    Unavailable(O),
    Success(T),
}

pub struct OrderedLinkedList<T, O: Ord> {
    head: Link<T, O>,
    tail: Link<T, O>,
    len: usize,
}

impl<T, O: Ord + Default + Copy> OrderedLinkedList<T, O> {
    pub fn new() -> Self {
        OrderedLinkedList {
            head: None,
            tail: None,
            len: 0
        }
    }

    #[allow(dead_code)]
    pub fn push(&mut self, elem: T) {
        self.push_by(elem, O::default());
    }

    #[allow(dead_code)]
    pub fn push_end(&mut self, elem: T) {
        let order = if let Some(end) = self.tail {
            unsafe { (*end.as_ref()).order }
        } else {
            O::default()
        };
        self.push_by(elem, order);
    }

    pub fn push_by(&mut self, elem: T, order: O) {
        // Try adding to tail
        if let Some(tail) = self.tail {
            unsafe {
                if (*tail.as_ptr()).order <= order {
                    let node = Node {
                        elem,
                        order,
                        next: None,
                    };
                    (*tail.as_ptr()).next = Some(NonNull::new_unchecked(Box::into_raw(Box::new(node))));
                    self.tail = (*tail.as_ptr()).next;
                    self.len += 1;
                    return;
                }
            }
        }

        // Try adding to somewhere between head and tail.
        if let Some(head) = self.head {
            unsafe {
                let mut cursor = head;
                if (*cursor.as_ptr()).order <= order {
                    loop {
                        if let Some(next) = (*cursor.as_ptr()).next {
                            if (*next.as_ptr()).order <= order {
                                cursor = next;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }

                    let node = Node {
                        elem,
                        order,
                        next: (*cursor.as_ptr()).next,
                    };
                    (*cursor.as_ptr()).next = Some(NonNull::new_unchecked(Box::into_raw(Box::new(node))));
                    self.len += 1;
                    return
                }
            }
        }

        // Try adding to the head.
        unsafe {
            let node = Node {
                elem,
                order,
                next: self.head,
            };
            let nn = NonNull::new_unchecked(Box::into_raw(Box::new(node)));
            self.head = Some(nn);
            if self.tail.is_none() {
                self.tail = Some(nn);
            }
            self.len += 1;
        }
    }

    #[allow(dead_code)]
    pub fn pop(&mut self) -> PoppedResult<T, O> {
        self.pop_aux(None)
    }

    #[allow(dead_code)]
    pub fn pop_by(&mut self, target_order: O) -> PoppedResult<T, O> {
        self.pop_aux(Some(target_order))
    }

    fn pop_aux(&mut self, target_order: Option<O>) -> PoppedResult<T, O> {
        if let Some(head) = self.head {
            let order = unsafe { (*head.as_ptr()).order };
            let can_pop = if let Some(target_order) = target_order {
                order <= target_order
            } else {
                true
            };
            if can_pop {
                let boxed = unsafe { Box::from_raw(head.as_ptr()) };
                self.head = boxed.next;
                self.len -= 1;
                if self.len == 0 {
                    self.tail = None;
                }
                PoppedResult::Success(boxed.elem)
            } else {
                PoppedResult::Unavailable(order)
            }
        } else {
            PoppedResult::Empty
        }
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T, O: Ord + Default + Copy> Default for OrderedLinkedList<T, O> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone, O: Clone + Ord + Default + Copy> Clone for OrderedLinkedList<T, O> {
    fn clone(&self) -> Self {
        let mut new_list = Self::new();
        let mut cursor = self.head;

        while let Some(node) = cursor {
            unsafe {
                new_list.push_by((*node.as_ptr()).elem.clone(), (*node.as_ptr()).order.clone());
                cursor = (*node.as_ptr()).next;
            }
        }
        new_list
    }
}

#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};
    use super::{OrderedLinkedList, PoppedResult};

    fn get_crt_time() -> u128 {
        SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went back!").as_millis()
    }
    
    #[test]
    fn it_works() {
        let mut mq: OrderedLinkedList<String, u128> = OrderedLinkedList::new();

        for i in 1..=3 {
            mq.push_by(format!("{}", i), get_crt_time() + i*10);
        }

        let mut mq2 = mq.clone();
        drop(mq);

        for _ in 1..=3 {
            mq2.push("0".to_string());
        }

        println!("--------------");

        while mq2.len() > 0 {
            get_result(&mut mq2)
        }
    }

    fn get_result(mq: &mut OrderedLinkedList<String, u128>) {
        match mq.pop_by(get_crt_time()) {
            PoppedResult::Empty => println!("EMPTY!!!"),
            PoppedResult::Unavailable(delay) => {
                let delay = delay - get_crt_time();
                println!("Delayed for {}ms", delay);
                sleep(Duration::from_millis(delay as u64))
            },
            PoppedResult::Success(handler) => {
                println!("Success, {}", handler);
            },
        }
    }
}