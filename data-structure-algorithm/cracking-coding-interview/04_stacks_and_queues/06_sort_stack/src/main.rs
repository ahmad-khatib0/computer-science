//
//
// Sort Stack: Write a program to sort a stack such that the smallest items are on the
// top. You can use an additional temporary stack, but you may not copy the elements
// into any other data structure (such as an array). The stack supports the following
// operations: push , pop, peek, and is Empty
//
//

use approach::{merge_sort, sort};
use rand::Rng;
mod approach;

fn main() {
    // Create and populate a stack
    let mut s = Vec::new();
    let mut rng = rand::rng();

    for _ in 0..10 {
        let r = rng.random_range(0..=1000);
        s.push(r);
    }

    println!("Original stack: {:?}", s);

    // Sort using insertion sort
    sort(&mut s);
    println!("After insertion sort:");
    while let Some(item) = s.pop() {
        println!("{}", item);
    }

    // Create another stack for merge sort
    let mut s2 = Vec::new();
    for _ in 0..10 {
        let r = rng.random_range(0..=1000);
        s2.push(r);
    }

    println!("\nOriginal stack for merge sort: {:?}", s2);
    let sorted = merge_sort(s2);
    println!("After merge sort:");
    for item in sorted {
        println!("{}", item);
    }
}
