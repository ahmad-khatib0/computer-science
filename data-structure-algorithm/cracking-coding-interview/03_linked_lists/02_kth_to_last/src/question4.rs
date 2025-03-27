#[derive(Debug)]
pub struct Node {
    pub data: i32,
    next: Option<Box<Node>>, // Using Box for ownership (singly linked list)
}

impl Node {
    fn new(data: i32) -> Self {
        Node { data, next: None }
    }
}

pub fn create_list_from_array(array: &[i32]) -> Option<Box<Node>> {
    if array.is_empty() {
        return None;
    }

    let mut head = Box::new(Node::new(array[0]));
    let mut current = &mut head;

    for &item in &array[1..] {
        current.next = Some(Box::new(Node::new(item)));
        current = current.next.as_mut().unwrap();
    }

    Some(head)
}

type Item = Option<Box<Node>>;

pub fn kth_to_last(head: &Item, k: i32) -> Item {
    // Example: use [0, 1, 2, 3] and find the 2nd-to-last element (which should be 2).
    // 1- p1 and p2 both point to the first node (0)
    let mut p1 = head.as_ref(); // Fast Pointer
    let mut p2 = head.as_ref(); // Slow Pointer

    // Moving p1 Ahead by k Nodes (k=2)
    // First iteration:  p1 moves from 0 to 1
    // Second iteration: p1 moves from 1 to 2
    // Now:
    //   p1 points to 2
    //   p2 still points to 0
    for _ in 0..k {
        p1?; // if none, return, out of bounds
        p1 = p1.unwrap().next.as_ref();
    }

    // Move both pointers until p1 reaches the end
    // First iteration:
    //     p1 moves from 2 to 3
    //     p2 moves from 0 to 1
    // Second iteration:
    //     p1 moves from 3 to None (end)
    //     p2 moves from 1 to 2 (our target)
    // Loop ends because p1 is None
    while let Some(node) = p1 {
        p1 = node.next.as_ref();
        p2 = p2.unwrap().next.as_ref();
    }

    // Convert the reference back to an owned Box (cloning the data)
    // p2 points to node 2
    // We create a new Box<Node> with the same data (2)
    // Returns Some(Box<Node { data: 2 }>)
    p2.map(|node| Box::new(Node::new(node.data)))
}
