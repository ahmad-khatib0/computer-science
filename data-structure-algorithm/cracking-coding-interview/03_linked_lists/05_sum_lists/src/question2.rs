use std::{cell::RefCell, rc::Rc};

use rust_lib::linked_list_node::LinkedListNode;

type Node = Rc<RefCell<LinkedListNode>>;

struct PartialSum {
    sum: Option<Node>,
    carry: i32,
}

impl PartialSum {
    fn new() -> Self {
        PartialSum {
            sum: None,
            carry: 0,
        }
    }
}

// Compute the length of a linked list
fn length(l: Option<Node>) -> i32 {
    match l {
        Some(node) => 1 + length(node.borrow().next.clone()),
        None => 0,
    }
}

// Recursive helper function to add two lists
fn add_lists_helper(l1: Option<Node>, l2: Option<Node>) -> PartialSum {
    if l1.is_none() && l2.is_none() {
        return PartialSum::new();
    }

    let next1 = l1.as_ref().and_then(|node| node.borrow().next.clone());
    let next2 = l2.as_ref().and_then(|node| node.borrow().next.clone());
    let mut sum = add_lists_helper(next1, next2);

    let val = sum.carry
        + l1.as_ref().map_or(0, |node| node.borrow().data)
        + l2.as_ref().map_or(0, |node| node.borrow().data);
    let full_result = insert_before(sum.sum.take(), val % 10);

    sum.sum = Some(full_result);
    sum.carry = val / 10;
    sum
}

// Adds two linked lists with proper padding
fn add_lists(l1: Option<Node>, l2: Option<Node>) -> Option<Node> {
    let len1 = length(l1.clone());
    let len2 = length(l2.clone());

    let l1 = if len1 < len2 {
        pad_list(l1, (len2 - len1) as usize)
    } else {
        l1
    };

    let l2 = if len2 < len1 {
        pad_list(l2, (len1 - len2) as usize)
    } else {
        l2
    };

    let sum = add_lists_helper(l1, l2);

    if sum.carry == 0 {
        sum.sum
    } else {
        Some(insert_before(sum.sum, sum.carry))
    }
}

// Pads the shorter list with zeros
// E,g Since 31 has 2 digits and 591 has 3 digits, we need to pad 31 with a
// leading 0 to make it 031:
fn pad_list(l: Option<Node>, padding: usize) -> Option<Node> {
    let mut head = l;
    for _ in 0..padding {
        head = Some(insert_before(head, 0));
    }
    head
}

// Inserts a node before the given list
fn insert_before(list: Option<Node>, data: i32) -> Node {
    let node = LinkedListNode::new(data);
    if let Some(existing) = list {
        node.borrow_mut().next = Some(existing);
    }
    node
}

// Converts a linked list to an integer
fn linked_list_to_int(node: Option<Node>) -> i32 {
    let mut value = 0;
    let mut current = node;

    while let Some(n) = current {
        value = value * 10 + n.borrow().data;
        current = n.borrow().next.clone();
    }

    value
}

pub fn list_sum() {
    let l_a1 = LinkedListNode::new(3);
    let l_a2 = LinkedListNode::new(1);
    l_a1.borrow_mut().next = Some(Rc::clone(&l_a2));

    let l_b1 = LinkedListNode::new(5);
    let l_b2 = LinkedListNode::new(9);
    let l_b3 = LinkedListNode::new(1);
    l_b1.borrow_mut().next = Some(Rc::clone(&l_b2));
    l_b2.borrow_mut().next = Some(Rc::clone(&l_b3));

    let list3 = add_lists(Some(Rc::clone(&l_a1)), Some(Rc::clone(&l_b1)));

    println!("  {}", l_a1.borrow().print_forward());
    println!("+ {}", l_b1.borrow().print_forward());
    println!("= {}", list3.as_ref().unwrap().borrow().print_forward());

    let l1 = linked_list_to_int(Some(Rc::clone(&l_a1)));
    let l2 = linked_list_to_int(Some(Rc::clone(&l_b1)));
    let l3 = linked_list_to_int(list3.clone());

    println!("{} + {} = {}", l1, l2, l3);
    println!("{} + {} = {}", l1, l2, l1 + l2);
}
