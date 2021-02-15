use ::rb_tree::{RBTree, IoTDevice};
use std::io;

const INVENTORY_MAX: u64 = 10000;

fn main() {
    let mut rb_tree = RBTree::new_empty();

    eprintln!("Building Innventory!");

    // Loop replacable with an excel/csv/disel-datbase getter function
    for i in 0..INVENTORY_MAX {
        rb_tree.add(IoTDevice::new(
            i,
            format!("Path{}", i),
            format!("Address{}", i)
        ));
    }

    eprintln!("Give me a number between 0 and 10000 and I'll give you that IoT Device!:\n");
    let mut uinput = String::new();
    io::stdin().read_line(&mut uinput).expect("Did not enter correct string");
    let uinput: u64 = match uinput.trim().parse() {
        Ok(val) => val,
        Err(e) => panic!("Bad number: {}\n{}", uinput, e)
    };
    let res = rb_tree.find(uinput);
    match res {
        Some(r) => eprintln!("{}", r),
        None => eprintln!("I'm sorry item: {} isn't in our Inventory", uinput)
    }
}
