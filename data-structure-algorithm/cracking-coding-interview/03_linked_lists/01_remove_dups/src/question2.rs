use std::{cell::RefCell, rc::Rc};

use rust_lib::linked_list_node::LinkedListNode;

pub fn delete_dups(head: Option<Rc<RefCell<LinkedListNode>>>) {
    let mut current = head.as_ref().map(Rc::clone);

    // .map(Rc::clone):
    // This does NOT clone the actual data (LinkedListNode or its contents). Instead, it:
    // Only increments the reference count of the Rc pointer (a cheap operation)
    // Creates a new pointer to the same heap allocation
    // Cost: Similar to copying a Java reference (very lightweight)
    //
    while let Some(current_node) = current {
        let current_data = current_node.borrow().data;
        let mut runner = current_node.borrow().next.as_ref().map(Rc::clone);

        while let Some(runner_node) = runner {
            let next_runner = runner_node.borrow().next.as_ref().map(Rc::clone);

            if runner_node.borrow().data == current_data {
                // Remove the duplicate node
                let next_node = runner_node.borrow().next.as_ref().map(Rc::clone);
                current_node.borrow_mut().next = next_node.clone();

                // // Update prev pointer if next exists
                if let Some(next) = next_node {
                    next.borrow_mut().prev = Some(Rc::downgrade(&current_node));
                }
            }

            // we always move to next_runner
            runner = next_runner;
        }

        current = current_node.borrow().next.as_ref().map(Rc::clone);
    }
}
