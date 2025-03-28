use std::{cell::RefCell, rc::Rc};

use rust_lib::linked_list_node::LinkedListNode;

type Item = Option<Rc<RefCell<LinkedListNode>>>;

pub fn partition(node: Item, x: i32) -> Item {
    let mut before_start: Item = None;
    let mut after_start: Item = None;
    let mut current = node;

    while let Some(current_node) = current {
        let next = current_node.borrow().next.clone();

        // Isolate the current node
        current_node.borrow_mut().next = None;

        if current_node.borrow().data < x {
            // Insert at head of before list
            current_node.borrow_mut().next = before_start.clone();
            before_start = Some(current_node.clone());
        } else {
            // Insert at head of after list
            current_node.borrow_mut().next = after_start.clone();
            after_start = Some(current_node.clone());
        }

        current = next;
    }

    // Merge before and after lists
    match before_start {
        None => after_start,
        Some(head) => {
            // Find tail of before list
            let mut tail = head.clone();
            loop {
                let next = tail.borrow().next.clone();
                match next {
                    Some(next_node) => {
                        tail = next_node;
                    }
                    None => break,
                }
            }

            // Connect to after list
            tail.borrow_mut().next = after_start;
            Some(head)
        }
    }
}
