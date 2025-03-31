use rust_lib::linked_list_node::LinkedListNode;
use std::{cell::RefCell, rc::Rc};

pub type NodeRef = Rc<RefCell<LinkedListNode>>;

pub fn create_looped_list(list_length: usize, k: usize) -> Option<NodeRef> {
    // E,g  list_length = 6, k = 3
    let mut nodes: Vec<NodeRef> = Vec::with_capacity(list_length);

    for i in 0..list_length {
        let node = LinkedListNode::new(i as i32);
        // If there is a previous node (prev), we link it to the new node.
        if let Some(prev) = nodes.last() {
            prev.borrow_mut().next = Some(node.clone());
        }
        nodes.push(node);
    }

    // Nodes = 0 -> 1 -> 2 -> 3 -> 4 -> 5 -> None

    // Create loop
    if list_length > k {
        if let Some(last) = nodes.last() {
            // If k = 3, list_length - k = 3, so we point the last node (5) to
            // the 3rd last node (3).
            // 0 -> 1 -> 2 -> 3 -> 4 -> 5
            //               ⬆-----------⬇
            last.borrow_mut().next = Some(nodes[list_length - k].clone());
        }
    }

    nodes.first().cloned()
}

pub fn find_beginning(head: Option<NodeRef>) -> Option<NodeRef> {
    let mut slow = head.clone();
    let mut fast = head.clone();

    // find meeting point
    while let (Some(ref slow_node), Some(ref fast_node)) = (slow.clone(), fast.clone()) {
        if let Some(ref next_fast) = fast_node.borrow().next {
            slow = slow_node.borrow().next.clone();
            fast = next_fast.borrow().next.clone();
        } else {
            return None; // No loop detected
        }

        if let (Some(s), Some(f)) = (slow.as_ref(), fast.as_ref()) {
            if Rc::ptr_eq(s, f) {
                break;
            }
        }
    }

    // Error check - no loop
    if fast.is_none() || fast.as_ref().unwrap().borrow().next.is_none() {
        return None;
    }

    // Move slow back to the start of the list.
    slow = head.clone();

    // Now both pointers move one step at a time.
    // When they meet again, that node is the start of the loop.
    //
    // SlowPointer starts from head.
    // FastPointer starts from collision point.
    while let (Some(s), Some(f)) = (slow.clone(), fast.clone()) {
        if Rc::ptr_eq(&s, &f) {
            return Some(s);
        }
        slow = s.borrow().next.clone();
        fast = f.borrow().next.clone();
    }

    None
}
