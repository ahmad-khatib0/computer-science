use std::{cell::RefCell, rc::Rc};

use rust_lib::linked_list_node::LinkedListNode;

type NodeRef = Rc<RefCell<LinkedListNode>>;

#[derive(Debug)]
struct Result {
    node: Option<NodeRef>,
    result: bool,
}

impl Result {
    pub fn new(node: Option<NodeRef>, result: bool) -> Self {
        Self { node, result }
    }
}

fn is_palindrome_recurse(head: Option<NodeRef>, length: i32) -> Result {
    // Even number of nodes
    if head.is_none() || length <= 0 {
        return Result::new(head, true);
    // If we are at the exact center of an odd-length list
    } else if length == 1 {
        let next = head.as_ref().unwrap().borrow().next.clone();
        return Result::new(next, true);
    }

    let next_node = head.as_ref().unwrap().borrow().next.clone();
    // This effectively reduces the problem by two nodes (one from each end).
    let res = is_palindrome_recurse(next_node, length - 2);

    // res.result is false (meaning a previous comparison already failed).
    // res.node is None (meaning we ran out of nodes to compare on the right side).
    if !res.result || res.node.is_none() {
        return res;
    }

    let head_node = head.as_ref().unwrap().borrow();
    let compare_node = res.node.as_ref().unwrap().borrow();
    // it compares left-side elements (head) with right-side elements (res.node).
    let result = head_node.data == compare_node.data;

    let next_res_node = res.node.as_ref().unwrap().borrow().next.clone();
    Result::new(next_res_node, result)
}

fn length_of_list(mut node: Option<NodeRef>) -> i32 {
    let mut size = 0;
    while let Some(n) = node {
        size += 1;
        node = n.borrow().next.clone();
    }
    size
}

pub fn is_palindrome(head: Option<NodeRef>) -> bool {
    let length = length_of_list(head.clone());
    let res = is_palindrome_recurse(head, length);
    res.result
}
