//
//
// Solution #1 : Recursive
//

use std::{cell::RefCell, rc::Rc};

use rust_lib::linked_list_node::LinkedListNode;

pub fn print_kth_to_last(head: Option<Rc<RefCell<LinkedListNode>>>, k: i32) -> i32 {
    match head {
        None => 0,
        Some(node) => {
            let idx = print_kth_to_last(node.borrow().next.clone(), k) + 1;
            if idx == k {
                println!("{}th to last node is {}", k, node.borrow().data);
            }
            idx
        }
    }
}
