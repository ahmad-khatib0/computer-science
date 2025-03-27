use std::cell::RefCell;
use std::rc::Rc;

use crate::node::Node;

fn nth_to_last_recursive(
    head: Option<Rc<RefCell<Node>>>,
    k: i32,
    counter: &mut i32,
) -> Option<Rc<RefCell<Node>>> {
    match head {
        None => None,
        Some(node) => {
            let result = nth_to_last_recursive(node.borrow().next.clone(), k, counter);
            *counter += 1;
            if *counter == k {
                Some(node)
            } else {
                result
            }
        }
    }
}

pub fn nth_to_last(head: Option<Rc<RefCell<Node>>>, k: i32) -> Option<Rc<RefCell<Node>>> {
    let mut counter = 0;
    nth_to_last_recursive(head, k, &mut counter)
}
