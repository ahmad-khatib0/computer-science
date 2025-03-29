use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct LinkedListNode {
    pub data: i32,
    pub next: Option<Rc<RefCell<LinkedListNode>>>,
    pub previous: Option<Rc<RefCell<LinkedListNode>>>,
}

impl LinkedListNode {
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

    // Clone the node and its successors
    pub fn clone(node: &Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        let node_ref = node.borrow();
        let next_clone = node_ref.next.as_ref().map(Self::clone);

        let new_node = Rc::new(RefCell::new(Self {
            data: node_ref.data,
            next: next_clone.clone(),
            previous: None, // We will fix this next
        }));

        // Set up previous references for the cloned nodes
        if let Some(next) = &next_clone {
            next.borrow_mut().previous = Some(new_node.clone());
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
        new_node.borrow_mut().previous = Some(Rc::clone(&current)); // FIXED: Use Rc, not Weak
        current.borrow_mut().next = Some(Rc::clone(&new_node));
        current = new_node;
    }

    Some(head)
}

impl LinkedListNode {
    pub fn new(data: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(LinkedListNode {
            data,
            next: None,
            previous: None,
        }))
    }

    pub fn set_next(&mut self, next: Option<Rc<RefCell<LinkedListNode>>>) {
        self.next = next;
    }

    pub fn set_previous(&mut self, prev: Option<Rc<RefCell<LinkedListNode>>>) {
        self.previous = prev;
    }

    pub fn print_forward(&self) -> String {
        let mut result = format!("{}", self.data);
        let mut current = self.next.clone();
        while let Some(node) = current {
            result.push_str(&format!("->{}", node.borrow().data));
            current = node.borrow().next.clone();
        }
        result
    }
}
