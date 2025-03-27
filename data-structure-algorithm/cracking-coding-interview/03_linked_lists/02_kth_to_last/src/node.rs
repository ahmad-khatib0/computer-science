use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct Node {
    pub next: Option<Rc<RefCell<Node>>>,
    pub data: i32,
}

impl Node {
    pub fn new(data: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { next: None, data }))
    }
}

pub fn print_list(head: &Option<Rc<RefCell<Node>>>) {
    let mut current = head.as_ref().map(Rc::clone);
    while let Some(node) = current {
        print!("{}", node.borrow().data);
        current = node.borrow().next.as_ref().map(Rc::clone);
    }
    println!();
}

pub fn create_list(count: i32) -> Option<Rc<RefCell<Node>>> {
    if count <= 0 {
        return None;
    }

    let head = Node::new(0);
    let mut last = Rc::clone(&head);

    for i in 1..count {
        let new_node = Node::new(i);
        last.borrow_mut().next = Some(Rc::clone(&new_node));
        last = new_node;
    }

    Some(head)
}
