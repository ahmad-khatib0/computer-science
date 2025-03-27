//
//
// Delete Middle Node: Im plement an algorithm to delete a node in the middle
// (i.e., any node but the first and last node, not necessarily the exact middle)
// of a singly linked list. given only access to that node.
// EXAMPLE
// In put: the node c from the linked list a -> b -> c -> d -> e -> f
// Result: nothing is returned, but the new linked list looks like a -> b -> d -> e -> f
//

use std::{cell::RefCell, rc::Rc};

use rust_lib::linked_list_node::LinkedListNode;

// I n this problem, you a re not given access to the head of the linked list. You only
// have access to that node. The solution is simply to copy the data from the next node
// over to the current node, and then to delete the next node.
pub fn delete_node(n: Option<Rc<RefCell<LinkedListNode>>>) -> bool {
    let n = match n {
        Some(n) => n,
        None => return false,
    };

    // Check if next exists in one borrow
    let next_exists = {
        let n_ref = n.borrow();
        n_ref.next.is_some()
    };

    if !next_exists {
        return false;
    }

    // Get next node data and next pointer
    let (next_data, new_next) = {
        let n_ref = n.borrow();
        let next_node = n_ref.next.as_ref().unwrap();
        let next_ref = next_node.borrow();
        (next_ref.data, next_ref.next.clone())
    };

    // Mutate current node
    {
        let mut n_mut = n.borrow_mut();
        n_mut.data = next_data;
        n_mut.next = new_next;
    }
    true
}

fn main() {
    println!("Hello, world!");
}
