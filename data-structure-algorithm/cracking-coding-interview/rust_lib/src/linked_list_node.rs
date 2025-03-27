use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct LinkedListNode {
    pub data: i32,
    pub next: Option<Rc<RefCell<LinkedListNode>>>,
    pub prev: Option<Weak<RefCell<LinkedListNode>>>,
    pub last: Option<Rc<RefCell<LinkedListNode>>>,
}

impl LinkedListNode {
    // Constructor with data only
    pub fn new(d: i32) -> Rc<RefCell<Self>> {
        let node = Rc::new(RefCell::new(Self {
            data: d,
            next: None,
            prev: None,
            last: None,
        }));
        node.borrow_mut().last = Some(node.clone());
        node
    }

    // Constructor with next and prev links
    pub fn new_with_links(
        d: i32,
        next: Option<Rc<RefCell<Self>>>,
        prev: Option<Rc<RefCell<Self>>>,
    ) -> Rc<RefCell<Self>> {
        let node = Self::new(d);
        {
            let mut node_mut = node.borrow_mut();
            node_mut.set_next(next);
            node_mut.set_previous(prev);
        }
        node
    }

    pub fn set_next(&mut self, next: Option<Rc<RefCell<Self>>>) {
        // Break old next relationship
        if let Some(old_next) = self.next.take() {
            old_next.borrow_mut().prev = None;
        }

        self.next = next.clone();

        // Update last pointer if this was the last node
        // Used Rc::ptr_eq to compare references (similar to == in Java)
        if let Some(last) = &self.last {
            if Rc::ptr_eq(
                last,
                &Rc::new(RefCell::new(Self {
                    data: self.data,
                    next: None,
                    prev: None,
                    last: None,
                })),
            ) {
                self.last = next.clone();
            }
        }

        // Establish new relationship
        if let Some(new_next) = &self.next {
            let mut new_next_mut = new_next.borrow_mut();
            new_next_mut.prev = Some(Rc::downgrade(&Rc::new(RefCell::new(Self {
                data: self.data,
                next: None,
                prev: None,
                last: None,
            }))));
        }
    }

    // Set previous node without cloning self
    pub fn set_previous(&mut self, prev: Option<Rc<RefCell<Self>>>) {
        self.prev = prev.as_ref().map(Rc::downgrade);

        if let Some(prev_node) = prev {
            let mut prev_node_mut = prev_node.borrow_mut();
            prev_node_mut.next = Some(Rc::new(RefCell::new(Self {
                data: self.data,
                next: None,
                prev: None,
                last: None,
            })));
        }
    }

    // Print the list forward
    pub fn print_forward(&self) -> String {
        if let Some(next) = &self.next {
            format!("{}->{}", self.data, next.borrow().print_forward())
        } else {
            self.data.to_string()
        }
    }

    // Clone the node and its successors
    pub fn clone(node: &Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        let node_ref = node.borrow();
        let next_clone = node_ref.next.as_ref().map(Self::clone);

        let new_node = Rc::new(RefCell::new(Self {
            data: node_ref.data,
            next: next_clone,
            prev: None,
            last: node_ref.last.as_ref().map(Self::clone),
        }));

        // Set up previous references for the cloned nodes
        if let Some(next) = &new_node.borrow().next {
            next.borrow_mut().prev = Some(Rc::downgrade(&new_node));
        }

        new_node
    }
}

pub fn create_linked_list_from_array(arr: &[i32]) -> Option<Rc<RefCell<LinkedListNode>>> {
    if arr.is_empty() {
        return None;
    }

    let head = LinkedListNode::new(arr[0]);
    let mut current = Rc::clone(&head);

    for &item in &arr[1..] {
        let new_node = LinkedListNode::new(item);
        current.borrow_mut().next = Some(Rc::clone(&new_node));
        current = new_node;
    }

    Some(head)
}
