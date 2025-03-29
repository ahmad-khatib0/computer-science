use std::{cell::RefCell, rc::Rc};

use rust_lib::linked_list_node::LinkedListNode;

fn add_lists(
    l1: Option<Rc<RefCell<LinkedListNode>>>,
    l2: Option<Rc<RefCell<LinkedListNode>>>,
) -> Option<Rc<RefCell<LinkedListNode>>> {
    add_lists_with_carry(l1, l2, 0)
}

fn add_lists_with_carry(
    l1: Option<Rc<RefCell<LinkedListNode>>>,
    l2: Option<Rc<RefCell<LinkedListNode>>>,
    carry: i32,
) -> Option<Rc<RefCell<LinkedListNode>>> {
    if l1.is_none() && l2.is_none() && carry == 0 {
        return None;
    }

    let mut value = carry;
    let next_l1 = l1.as_ref().and_then(|node| {
        value += node.borrow().data;
        node.borrow().next.clone()
    });
    let next_l2 = l2.as_ref().and_then(|node| {
        value += node.borrow().data;
        node.borrow().next.clone()
    });

    let result_node = LinkedListNode::new(value % 10);
    let next_node = add_lists_with_carry(next_l1, next_l2, if value >= 10 { 1 } else { 0 });
    result_node.borrow_mut().set_next(next_node);

    Some(result_node)
}

fn linked_list_to_int(node: Option<Rc<RefCell<LinkedListNode>>>) -> i32 {
    if let Some(n) = node {
        let next_value = linked_list_to_int(n.borrow().next.clone());
        next_value * 10 + n.borrow().data
    } else {
        0
    }
}

pub fn list_sum() {
    let l_a1 = LinkedListNode::new(9);
    let l_a2 = LinkedListNode::new_with_links(9, Some(l_a1.clone()), None);
    let l_a3 = LinkedListNode::new_with_links(9, Some(l_a2.clone()), None);

    let l_b1 = LinkedListNode::new(1);
    let l_b2 = LinkedListNode::new_with_links(0, Some(l_b1.clone()), None);
    let l_b3 = LinkedListNode::new_with_links(0, Some(l_b2.clone()), None);

    let list3 = add_lists(Some(l_a1.clone()), Some(l_b1.clone()));

    println!(
        "  {} + {} = {}",
        l_a1.borrow().print_forward(),
        l_b1.borrow().print_forward(),
        list3
            .as_ref()
            .map(|n| n.borrow().print_forward())
            .unwrap_or_else(|| "0".to_string())
    );

    let l1 = linked_list_to_int(Some(l_a1));
    let l2 = linked_list_to_int(Some(l_b1));
    let l3 = linked_list_to_int(list3);

    println!("{} + {} = {}", l1, l2, l3);
    println!("{} + {} = {}", l1, l2, l1 + l2);
}
