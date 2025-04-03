mod approach1;
mod approach2;
mod node_with_min;

//
// Stack Min: How would you design a stack which, in addition to push and pop, has a
// function min which retu rns the minimum element? P u s h, pop and min should all
// operate in 0(1) time.
//

fn main() {
    let mut stack = approach1::StackWithMin::new();
    let mut stack2 = approach2::StackWithMin::new();
    let array = [2, 1, 3, 1];

    // Push values
    for &value in &array {
        stack.push(value);
        stack2.push(value);
        print!("{}, ", value);
    }

    println!("\n");

    // Pop values and show mins
    for _ in 0..array.len() {
        let popped1 = stack.pop().unwrap();
        let popped2 = stack2.pop().unwrap();
        println!("Popped {}, {}", popped1, popped2);
        println!("New min is {}, {}", stack.min(), stack2.min());
    }
}
