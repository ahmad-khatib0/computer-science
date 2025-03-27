use std::{cell::RefCell, rc::Rc};

use crate::node::Node;

// Wrapper for the index counter (equivalent to Java's Index class)
#[derive(Default)]
pub struct Index {
    value: i32,
}

type Item = Option<Rc<RefCell<Node>>>;

pub fn kth_to_last(head: Item, k: i32, idx: &mut Index) -> Item {
    match head {
        None => None,
        Some(node) => {
            let result = kth_to_last(node.borrow().next.clone(), k, idx);
            idx.value += 1;
            if idx.value == k {
                Some(node)
            } else {
                result
            }
        }
    }
}
