use std::rc::Rc;

use rust_lib::linked_list_node::LinkedListNode;

mod question1;
mod question2;

//
// Remove Dups: Write code to remove duplicates from an unsorted linked list.
// FOLLOW U P
// How wou ld you solve t h i s problem i f a tem porary buffer i s not al lowed?
//

fn main() {
    let first = LinkedListNode::new(0);
    let head = Some(first.clone());
    let mut current = first.clone();

    for i in 0..8 {
        let new_node = LinkedListNode::new(i % 2);
        current.borrow_mut().set_next(Some(new_node.clone()));
        new_node.borrow_mut().prev = Some(Rc::downgrade(&current));
        current = new_node;
    }

    println!(
        "Before: {}",
        head.as_ref().unwrap().borrow().print_forward()
    );

    // question1::delete_dups(head.clone());
    question2::delete_dups(head.clone());

    // println!("After: {}", head.as_ref().unwrap().borrow().print_forward());
    println!("After: {}", head.as_ref().unwrap().borrow().print_forward());
}
