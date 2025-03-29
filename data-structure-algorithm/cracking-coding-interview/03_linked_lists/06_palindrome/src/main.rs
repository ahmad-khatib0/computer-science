//
// Palindrome: Implement a fu n ctio n to check if a linked list is a pa lindrome.
//

// use question2::create_list;
use rust_lib::linked_list_node::LinkedListNode;
use std::{cell::RefCell, rc::Rc};

mod question1;
mod question2;
mod question3;

type Node = Option<Rc<RefCell<LinkedListNode>>>;

fn create_list() -> (Node, Node) {
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

    let head = Some(nodes[0].clone());
    let rev = reverse_and_clone(&head);
    (head, rev)
}

fn reverse_and_clone(node: &Node) -> Node {
    let mut head: Node = None;
    let mut current = node.clone();

    while let Some(n) = current {
        let new_node = LinkedListNode::new(n.borrow().data);
        new_node.borrow_mut().set_next(head.clone());
        head = Some(new_node);
        current = n.borrow().next.clone();
    }
    head
}

fn main() {
    let head1 = create_list();

    println!(
        "list 1: {}, and list 2: {}, are palindrome = {}",
        head1.0.as_ref().unwrap().borrow().print_forward(),
        head1.1.as_ref().unwrap().borrow().print_forward(),
        question1::is_palindrome(&head1.0, &head1.1),
    );

    let head2 = create_list();
    println!(
        "List 1: {}, is palindrome = {}",
        head2.0.as_ref().unwrap().borrow().print_forward(),
        question2::is_palindrome(&head2.0),
    );

    let head3 = create_list();
    let list3 = head3.0.as_ref().unwrap().borrow().print_forward();
    println!(
        "List: {}, is palindrome = {}",
        list3,
        question3::is_palindrome(head3.0),
    );
}
