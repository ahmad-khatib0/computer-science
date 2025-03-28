use std::{cell::RefCell, rc::Rc};

use rust_lib::linked_list_node::LinkedListNode;

type Item = Option<Rc<RefCell<LinkedListNode>>>;

pub fn partition(node: Item, x: i32) -> Item {
    // before_start: Head of the "<x" partition
    let mut before_start: Item = None;
    // before_end: Tail of the "<x" partition (needed to efficiently append new nodes)
    let mut before_end: Item = None;
    // after_start: Head of the "≥x" partition
    let mut after_start: Item = None;
    // after_end: Tail of the "≥x" partition
    let mut after_end: Item = None;

    let mut current = node;
    while let Some(current_node) = current {
        let next = current_node.borrow().next.clone();

        // Isolate the current node
        current_node.borrow_mut().next = None;
        if current_node.borrow().data < x {
            if before_start.is_none() {
                before_start = Some(current_node.clone());
                before_end = before_start.clone();
            } else if let Some(end) = &before_end {
                end.borrow_mut().next = Some(current_node.clone());
                before_end = Some(current_node.clone());
            }
        } else {
            if after_start.is_none() {
                after_start = Some(current_node.clone());
                after_end = after_start.clone();
            } else {
                if let Some(end) = &after_end {
                    end.borrow_mut().next = Some(current_node.clone());
                    after_end = Some(current_node.clone());
                }
            }
        }

        current = next;
    }

    // Merge before and after lists
    if before_start.is_none() {
        return after_start;
    }

    if let Some(end) = before_end {
        end.borrow_mut().next = after_start;
    }

    before_start
}
