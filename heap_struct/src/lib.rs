// IoTDevice Message Notification Que

use std::fmt;
use std::mem;
use std::boxed::Box;

#[derive(Debug, Clone)]
pub struct IoTDevice {
    numerical_id: u64,
    address: String,
    path: String
}

impl IoTDevice {
    pub fn new(numerical_id: u64, address: String, path: String) -> IoTDevice {
        IoTDevice {
            numerical_id,
            address,
            path
        }
    } 
}

impl fmt::Display for IoTDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Id #{}\nAddress: {}\nPath: {}", self.numerical_id, self.address, self.path)
    }
}

#[derive(Debug, Clone)]
pub struct MessageNotification {
    pub no_messages: u64,
    pub dev: IoTDevice
}

impl MessageNotification {
    pub fn new(no_messages: u64, dev: IoTDevice) -> MessageNotification {
        MessageNotification {
            no_messages,
            dev
        }
    }
}

pub struct MessageChecker {
    pub length: usize,
    heap: Vec<Box<MessageNotification>>
}

impl MessageChecker {
    pub fn new_empty() -> MessageChecker {
        MessageChecker {
            length: 0,
            heap: vec![]
        }
    }

    pub fn add(&mut self, notification: MessageNotification) {
        self.heap.push(Box::new(notification));
        self.length = self.heap.len();
        if self.length > 1 {
            let mut i = self.length;
            while i / 2 > 0 && self.has_more_messages(i, i / 2) {
                self.swap(i, i / 2);
                i /= 2;
            }
        }
    }

    pub fn pop(&mut self) -> Option<MessageNotification> {
        if self.length > 0 {
            let elem = self.heap.swap_remove(0);
            self.length = self.heap.len();
            let mut i = 1;
            while i  * 2 < self.length {
                let children = (i * 2, i * 2 + 1);
                i = if self.has_more_messages(children.0, children.1) {
                    if self.has_more_messages(children.0, i) {
                        self.swap(i, children.0);
                        children.0
                    } else {
                        break;
                    }
                } else {
                    if self.has_more_messages(children.1, i) {
                        self.swap(i, children.1);
                        children.1
                    } else {
                        break;
                    }
                }
            }
            Some(*elem)
        } else {
            None
        }
    }

    fn has_more_messages(&self, pos1: usize, pos2: usize) -> bool {
        let a = &self.heap[pos1 - 1];
        let b = &self.heap[pos2 - 1];
        a.no_messages >= b.no_messages
    }

    fn swap(&mut self, pos1: usize, pos2: usize) {
        let m2 = self.heap[pos1 - 1].clone();
        self.heap[pos1 - 1] = mem::replace(&mut self.heap[pos2 - 1], m2)
    }
}