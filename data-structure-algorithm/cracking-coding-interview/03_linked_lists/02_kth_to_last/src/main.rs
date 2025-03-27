//
//
// Return Kth to Last: Implement an a lgorithm to find the kth to
// last element of a singly linked list.
//
// Key Observations:
//     k = 1 → Returns the last element (tail).
//     k = length of list → Returns the first element (head).
//     k > length of list → Out of bounds (should return None or handle gracefully).
//

use rust_lib::linked_list_node::create_linked_list_from_array;

mod node;
mod question1;
mod question2;
mod question3;
mod question4;

fn main() {
    let array = [0, 1, 2, 3, 4, 5, 6];
    let head = create_linked_list_from_array(&array);

    for i in 0..=array.len() + 1 {
        question1::print_kth_to_last(head.clone(), i as i32);
    }

    let count = 5;
    let head = node::create_list(count);
    node::print_list(&head);

    for k in 0..=count {
        if let Some(node) = question2::nth_to_last(head.clone(), k) {
            println!("{}: {}", k, node.borrow().data);
        }
    }

    for i in 0..=array.len() + 1 {
        let mut idx = question3::Index::default();
        let node = question3::kth_to_last(head.clone(), i as i32, &mut idx);
        let node_value = match node {
            Some(n) => n.borrow().data.to_string(),
            None => "null".to_string(),
        };
        println!("{}: {}", i, node_value);
    }

    let arr2 = [0, 1, 2, 3, 4, 5, 6];
    let head2 = question4::create_list_from_array(&arr2);
    for i in 0..=arr2.len() + 1 {
        let node = question4::kth_to_last(&head2, i as i32);
        let node_value = match node {
            Some(n) => n.data.to_string(),
            None => "null".to_string(),
        };
        println!("{}: {}", i, node_value);
    }
}
