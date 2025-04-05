mod approach;

//
// Stack of Plates: I magine a (literal) stack of plates. If the stack gets too high,
// it might topple. Therefore, in real life, we would likely start a new stack when
// the previous stack exceeds some threshold. Implement a data structure SetOfStacks
// that mimics this. SetOfSta c ks should be composed of several stacks and should create
// a new stack once the previous one exceeds capacity. SetOfStacks . push( ) and
// SetOfStacks. pop() should behave identically to a single stack (that is, pop()
// should return the same values as it would if there were just a single stack).
//
// FOLLOW UP
// Implement a function popAt(int index) which performs a pop
// operation on a specific sub­stack
//

fn main() {
    let capacity_per_substack = 5;
    let mut set = approach::SetOfStacks::new(capacity_per_substack);

    let length = 10;
    // Push values 0..34 onto the stack
    for i in 0..length {
        set.push(i);
    }

    // Pop 35 times (one more than we pushed to test edge case)
    for _ in 0..length {
        match set.pop() {
            Some(val) => println!("Popped {}", val),
            None => println!("Stack is empty!"),
        }
    }
}
