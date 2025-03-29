use std::{cell::RefCell, rc::Rc};

use rust_lib::linked_list_node::LinkedListNode;

pub fn partition(
    node: Option<Rc<RefCell<LinkedListNode>>>,
    x: i32,
) -> Option<Rc<RefCell<LinkedListNode>>> {
    let mut head = node.clone();
    let mut tail = node.clone();
    let mut current = node;

    while let Some(current_node) = current {
        let next = current_node.borrow().next.clone();

        // Clear the node's connections before moving it
        current_node.borrow_mut().next = None;
        current_node.borrow_mut().prev = None;

        if current_node.borrow().data < x {
            // Insert at head
            if let Some(head_node) = head.take() {
                current_node.borrow_mut().next = Some(head_node.clone());
                head_node.borrow_mut().prev = Some(Rc::downgrade(&current_node));
            }
            head = Some(current_node.clone());
            if tail.is_none() {
                tail = head.clone();
            }
        } else {
            // Insert at tail
            if let Some(tail_node) = tail.take() {
                tail_node.borrow_mut().next = Some(current_node.clone());
                current_node.borrow_mut().prev = Some(Rc::downgrade(&tail_node));
                tail = Some(current_node.clone());
            } else {
                head = Some(current_node.clone());
                tail = head.clone();
            }
        }

        current = next;
    }

    head
}
