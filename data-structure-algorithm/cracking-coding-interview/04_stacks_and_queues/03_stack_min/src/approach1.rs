use std::cmp;

use crate::node_with_min::NodeWithMin;

// let's wal k through it with a short example:
// push ( 5 ) ; // stack is { 5 } , min is 5
// pus h ( 6 ) ; / / stack is {6, 5 } , min i s 5
// push ( 3 ) ; / / stack i s { 3 , 6 , 5 } , min is 3
// pus h ( 7 ) ; / / stack i s { 7 , 3 , 6, 5 } , min i s 3
// pop ( ) ; / / pop s 7 . stack is {3, 6, 5 } , min i s 3
// pop ( ) ; // pops 3 . stack is {6, 5 } . min i s 5 .
//
// Observe how once the stack goes back to a prior state ( { 6 , 5 } ), the minimum also
// goes back to its prior state (5). This leads us to our second solution. If we kept
// track of the minimum at each state, we would be able to easily know the minimum.
// We can do this by having each node record what the minimum beneath itself is. Then,
// to find the min, you just look at what the top element thinks is the min.
//

pub struct StackWithMin {
    stack: Vec<NodeWithMin>,
}

impl StackWithMin {
    pub fn new() -> Self {
        StackWithMin { stack: Vec::new() }
    }

    // When you push an element onto the stack, the element is given the
    // current minimum. It sets its "local min " to be the min.
    pub fn push(&mut self, value: i32) {
        let new_min = cmp::min(value, self.min());
        self.stack.push(NodeWithMin::new(value, new_min))
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.stack.pop().map(|n| n.value)
    }

    pub fn min(&self) -> i32 {
        if self.stack.is_empty() {
            i32::MAX
        } else {
            self.stack.last().unwrap().min
        }
    }
}

pub fn stack_min() {
    let mut stack = StackWithMin::new();
    let array = [2, 1, 3, 1];

    for &value in &array {
        stack.push(value);
        print!("{}, ", value);
    }

    println!("\n");
    for _ in 0..array.len() {
        let popped = stack.pop().unwrap();
        println!("Popped {}", popped);
        println!("New min is {}", stack.min());
    }
}
