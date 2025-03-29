use std::{cell::RefCell, rc::Rc};

use rust_lib::linked_list_node::LinkedListNode;

type Node = Option<Rc<RefCell<LinkedListNode>>>;

pub fn is_palindrome(one: &Node, two: &Node) -> bool {
    let mut one_current = one.clone();
    let mut two_current = two.clone();

    while let (Some(n1), Some(n2)) = (one_current.clone(), two_current.clone()) {
        if n1.borrow().data != n2.borrow().data {
            return false;
        }
        one_current = n1.borrow().next.clone();
        two_current = n2.borrow().next.clone();
    }
    one_current.is_none() && two_current.is_none()
}
