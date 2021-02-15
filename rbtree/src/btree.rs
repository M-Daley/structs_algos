#![allow(dead_code)]

use std::mem;
use std::fmt;

type Tree = Option<Box<Node>>;

struct Node {
    pub dev: IoTDevice,
    left: Tree,
    right: Tree,
}

impl Node {
    fn new(device: IoTDevice) -> Tree {
        Some(Box::new(Node {
            dev: device,
            left: None,
            right: None,
        }))
    }
}

#[derive(Clone, Debug)]
pub struct IoTDevice {
    pub numerical_id: u64,
    pub path: String,
    pub address: String,
}

impl IoTDevice {
    pub fn new(numerical_id: u64, address: impl Into<String>, path: impl Into<String>) -> IoTDevice {
        IoTDevice {
            address: address.into(),
            numerical_id,
            path: path.into()
        }
    }
}

impl PartialEq for IoTDevice {
    fn eq(&self, other: &IoTDevice) -> bool {
        self.numerical_id == other.numerical_id && self.address == other.address
    }
}

impl fmt::Display for IoTDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Id #{}\nPath: {}\nAddress: {}", self.numerical_id, self.path, self.address)
    }
}

pub struct BinarySearchTree {
    root: Tree,
    pub length: u64,
}

impl BinarySearchTree {
    pub fn new_empty() -> BinarySearchTree {
        BinarySearchTree {
            root: None,
            length: 0,
        }
    }

    pub fn add(&mut self, device: IoTDevice) {
        self.length += 1;
        let root = mem::replace(&mut self.root, None);
        self.root = self.add_rec(root, device)
    }

    fn add_rec(&mut self, node: Tree, device: IoTDevice) -> Tree {
        match node {
            Some(mut n) => {
                if n.dev.numerical_id <= device.numerical_id {
                    n.left = self.add_rec(n.left, device);
                    Some(n)
                } else {
                    n.right = self.add_rec(n.right, device);
                    Some(n)
                }
            }
            _ => Node::new(device),
        }
    }

    pub fn find(&self, numerical_id: u64) -> Option<IoTDevice> {
        self.find_r(&self.root, numerical_id)
    }

    fn find_r(&self, node: &Tree, numerical_id: u64) -> Option<IoTDevice> {
        match node {
            Some(n) => {
                if n.dev.numerical_id == numerical_id {
                    Some(n.dev.clone())
                } else if n.dev.numerical_id < numerical_id {
                    self.find_r(&n.left, numerical_id)
                } else {
                    self.find_r(&n.right, numerical_id)
                }
            }
            _ => None,
        }
    }

    pub fn walk(&self, callback: impl Fn(&IoTDevice) -> ()) {
        self.walk_in_order(&self.root, &callback)
    }

    fn walk_in_order(&self, node: &Tree, callback: &impl Fn(&IoTDevice) -> ()) {
        if let Some(n) = node {
            self.walk_in_order(&n.left, callback);
            callback(&n.dev);
            self.walk_in_order(&n.right, callback)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[test]
    fn walk() {
        let tree = BinarySearchTree::new_empty();
        let my_devices: RefCell<Vec<IoTDevice>> = RefCell::new(vec![]);
        tree.walk(|n| my_devices.borrow_mut().push(n.clone()));
    }
}