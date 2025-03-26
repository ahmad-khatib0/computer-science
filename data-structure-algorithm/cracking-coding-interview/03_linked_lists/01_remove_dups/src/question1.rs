use std::{cell::RefCell, collections::HashSet, rc::Rc};

use rust_lib::linked_list_node::LinkedListNode;

pub fn delete_dups(head: Option<Rc<RefCell<LinkedListNode>>>) {
    let mut set = HashSet::new();
    let mut current = head.clone();
    let mut previous: Option<Rc<RefCell<LinkedListNode>>> = None;

    while let Some(current_node) = current {
        let current_data = current_node.borrow().data;

        if set.contains(&current_data) {
            // Remove duplicate - skip current node
            if let Some(prev_node) = &previous {
                let next = current_node.borrow().next.clone();
                prev_node.borrow_mut().next = next;
            }
        } else {
            // New value - keep it and update previous
            set.insert(current_data);
            previous = Some(current_node.clone());
        }

        current = current_node.borrow().next.clone();
    }
}
