use rust_lib::linked_list_node::LinkedListNode;
use std::collections::VecDeque;
use std::{cell::RefCell, rc::Rc};

type Node = Option<Rc<RefCell<LinkedListNode>>>;

pub fn create_list() -> Node {
    let length = 9;
    let mut nodes: Vec<Rc<RefCell<LinkedListNode>>> = Vec::with_capacity(length);

    // Create nodes
    for i in 0..length {
        let data = if i >= length / 2 { length - i - 1 } else { i };
        nodes.push(LinkedListNode::new(data as i32));
    }

    // Set next and previous pointers
    for i in 0..length {
        if i < length - 1 {
            nodes[i].borrow_mut().set_next(Some(nodes[i + 1].clone()));
        }
        if i > 0 {
            nodes[i]
                .borrow_mut()
                .set_previous(Some(nodes[i - 1].clone()));
        }
    }

    Some(nodes[0].clone()) // Return the head of the linked list
}

pub fn is_palindrome(head: &Node) -> bool {
    let mut fast = head.clone();
    let mut slow = head.clone();
    let mut stack: VecDeque<i32> = VecDeque::new();

    // Move fast by 2 steps and slow by 1 step, pushing half elements into the stack
    while let (Some(f_node), Some(s_node)) = (fast.clone(), slow.clone()) {
        stack.push_back(s_node.borrow().data);

        slow = s_node.borrow().next.clone();
        fast = f_node
            .borrow()
            .next
            .as_ref()
            .and_then(|n| n.borrow().next.clone());
    }

    // If the list has an odd number of elements, skip the middle one
    if fast.is_some() {
        slow = slow.and_then(|n| n.borrow().next.clone());
    }

    // Compare the second half with the first half (stored in stack)
    while let Some(s_node) = slow {
        let top = stack.pop_back().unwrap();
        if top != s_node.borrow().data {
            return false;
        }

        slow = s_node.borrow().next.clone();
    }
    true
}
