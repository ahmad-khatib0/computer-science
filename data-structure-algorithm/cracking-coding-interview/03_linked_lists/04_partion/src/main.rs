// Partition: Write code to partition a linked list around a value x, such that all
// nodes less than x come before all nodes greater than or equal to x. If x is contained
// within the list, the values of x only need to be after the elements less than x
// (see below). The partition element x can appear anywhere in the "right partition";
// it does not need to appear between the left a nd right partitions.
//
// EXAMPLE
// In put: 3 - > 5 -> 8 -> 5 - > 10 - > 2 - > 1 [partition = 5]
// Output: 3 - > 1 - > 2 - > 10 - > 5 - > 5 - > 8

use rust_lib::linked_list_node::LinkedListNode;
use std::{cell::RefCell, rc::Rc};

mod question1;
mod question2;
mod question3;

fn create_list() -> Rc<RefCell<LinkedListNode>> {
    // let vals = [33, 9, 2, 3, 10, 10389, 838, 874578, 5];
    let vals = [33, 9, 2, 3, 10, 10389, 838, 874578, 5];
    let head = LinkedListNode::new(vals[0]);
    let mut current = head.clone();

    for &val in vals.iter().skip(1) {
        let new_node = LinkedListNode::new(val);
        new_node.borrow_mut().prev = Some(Rc::downgrade(&current));
        current.borrow_mut().next = Some(new_node.clone());
        current = new_node;
    }

    head
}

fn main() {
    let head1 = create_list();
    println!("{}", head1.borrow().print_forward());
    let partitioned_head1 = question1::partition(Some(head1), 10);
    if let Some(h) = partitioned_head1 {
        println!("{}", h.borrow().print_forward());
    }

    let head2 = create_list();
    println!("{}", head2.borrow().print_forward());
    let partitioned_head2 = question2::partition(Some(head2), 10);
    if let Some(h) = partitioned_head2 {
        println!("{}", h.borrow().print_forward());
    }

    let head3 = create_list();
    println!("{}", head3.borrow().print_forward());
    let partitioned_head3 = question3::partition(Some(head3), 10);
    if let Some(h) = partitioned_head3 {
        println!("{}", h.borrow().print_forward());
    }
}
