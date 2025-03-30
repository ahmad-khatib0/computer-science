use std::{cell::RefCell, rc::Rc};

use rust_lib::linked_list_node::LinkedListNode;

pub type NodeRef = Rc<RefCell<LinkedListNode>>;

struct Result {
    pub tail: Option<NodeRef>,
    pub size: usize,
}

impl Result {
    pub fn new(tail: Option<NodeRef>, size: usize) -> Self {
        Self { tail, size }
    }
}

fn get_tail_and_size(list: Option<NodeRef>) -> Option<Result> {
    let mut current = list.clone();
    let mut size = 0;

    while let Some(node) = current.clone() {
        size += 1;
        if node.borrow().next.is_none() {
            return Some(Result::new(Some(node), size));
        }
        current = node.borrow().next.clone();
    }
    Some(Result::new(None, size))
}

pub fn get_kth_node(mut head: Option<NodeRef>, k: usize) -> Option<NodeRef> {
    for _ in 0..k {
        head = head?.borrow().next.clone();
    }
    head
}

pub fn find_intersection(list1: Option<NodeRef>, list2: Option<NodeRef>) -> Option<NodeRef> {
    if list1.is_none() || list2.is_none() {
        return None;
    }

    // get tail and sizes
    let res1 = get_tail_and_size(list1.clone())?;
    let res2 = get_tail_and_size(list2.clone())?;

    // If different tail nodes, then there's no intersection.
    if !Rc::ptr_eq(res1.tail.as_ref()?, res2.tail.as_ref()?) {
        return None;
    }

    // Set pointers to the start of each linked list.
    let (mut shorter, mut longer) = if res1.size < res2.size {
        (list1, list2)
    } else {
        (list2, list1)
    };

    // Advance the pointer for the longer linked list by the difference in lengths.
    longer = get_kth_node(
        longer,
        (res1.size as isize - res2.size as isize).unsigned_abs(),
    );

    while let (Some(s), Some(l)) = (shorter.clone(), longer.clone()) {
        if Rc::ptr_eq(&s, &l) {
            // Return either one.
            return Some(l);
        }
        shorter = shorter?.borrow().next.clone();
        longer = longer?.borrow().next.clone();
    }

    None
}
