//
//
// Loop Detection: Given a circular linked list, implement an algorithm that
// returns the node at the beginning of the loop.
// DEFINITION
// Circular linked list: A (corrupt) linked list in which a node's next pointer
// points to an earlier node, so as to make a loop in the linked list.
// EXAMPLE
// Input: A - > B -> C -> D -> E -> C [the same C as earlier]
// Output: c
//

use question::{create_looped_list, find_beginning};

mod question;

fn main() {
    let list_length = 100;
    let k = 10;

    let head = create_looped_list(list_length, k);
    let loop_node = find_beginning(head);

    match loop_node {
        Some(node) => println!("Cycle starts at: {}", node.borrow().data),
        None => println!("No Cycle."),
    }
}
