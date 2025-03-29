// Sum Lists: You have two num bers represented by a linked list, where each node
// contains a single digit. The dig its are stored in reverse order, such that the
// 1 's digit is at the head of the list. Write a function that adds the two numbers
// and returns the sum as a linked list.
// EXAMPLE
// Input: (7 -> 1 -> 6) + ( 5 - > 9 - > 2 ) . That is, 617 + 295.
// Output: 2 -> 1 -> 9. That is, 912 .
// FOLLOW UP
// Suppose the d gits are stored in forward order. Repeat the above problem.
// Input: (6 -> 1 -> 7) + ( 2 -> 9 -> 5 ) . That is, 617 + 295.
// Output: 9 -> 1 - > 2. That is, 9 1 2 .
//

mod question1;
mod question2;

fn main() {
    question1::list_sum();

    question2::list_sum();
}
