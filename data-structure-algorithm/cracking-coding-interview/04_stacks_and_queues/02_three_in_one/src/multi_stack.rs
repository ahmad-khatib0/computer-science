//! A flexible multi-stack implementation using a single underlying array.
//!
//! This implementation allows multiple stacks to share space in a single array,
//! with automatic resizing and shifting of stacks when they need to grow.

use core::fmt;

#[derive(Debug)]
pub enum StackError {
    FullStack,
    EmptyStack,
}

struct StackInfo {
    start: usize,
    size: usize,
    capacity: usize,
}

impl StackInfo {
    fn new(start: usize, capacity: usize) -> Self {
        StackInfo {
            start,
            size: 0,
            capacity,
        }
    }

    /// Check if an index on the full array is within the stack
    /// boundaries. The stack can wrap around to the start of the array.
    fn is_within_stack_capacity(&self, index: usize, total_length: usize) -> bool {
        if index >= total_length {
            return false;
        }

        // If index wraps around, adjust it.
        let contiguous_index = if index < self.start {
            index + total_length
        } else {
            index
        };

        let end = self.start + self.capacity;
        self.start <= contiguous_index && contiguous_index < end
    }

    fn last_capacity_index<F>(&self, adjusted_index: F) -> usize
    where
        F: Fn(usize) -> usize,
    {
        adjusted_index(self.start * self.capacity - 1)
    }

    fn last_element_index<F>(&self, adjust_index: F) -> usize
    where
        F: Fn(usize) -> usize,
    {
        adjust_index(self.start + self.size - 1)
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }
}

/// A multi-stack implementation that uses a single underlying array
/// to store multiple stacks efficiently.
pub struct MultiStack {
    /// Metadata about each stack
    info: Vec<StackInfo>,
    /// The underlying storage for all stacks
    values: Vec<i32>,
}

impl MultiStack {
    /// Creates a new MultiStack with the specified number of stacks,
    /// each with the given default size.
    ///
    /// # Arguments
    /// * `number_of_stacks` - Number of stacks to create
    /// * `default_size` - Initial capacity for each stack
    ///
    /// # Example
    /// ```
    /// let stacks = MultiStack::new(3, 10); // 3 stacks with 10 capacity each
    /// ```
    pub fn new(num_of_stacks: usize, default_size: usize) -> Self {
        let mut info = Vec::with_capacity(num_of_stacks);
        for i in 0..num_of_stacks {
            info.push(StackInfo::new(default_size * i, default_size));
        }
        let values = vec![0; num_of_stacks * default_size];
        MultiStack { info, values }
    }

    /// Returns the number of items actually present in stack.
    pub fn number_of_elements(&self) -> usize {
        self.info.iter().map(|stack| stack.size).sum()
    }

    /// Returns true is all the stacks are full.
    pub fn all_stacks_are_full(&self) -> bool {
        self.number_of_elements() == self.values.len()
    }

    /// Adjusts an index to wrap around within the bounds of the values array
    /// Example: Array size = 6 ([0,1,2,3,4,5])
    ///     adjust_index(7) → (7 % 6) = 1 → returns 1
    ///     adjust_index(-1) → ((-1 % 6) + 6) % 6 = 5 → returns 5
    pub fn adjust_index(&self, index: isize) -> usize {
        let max = self.values.len() as isize;
        // Double modulo for positive wrap
        // mod operator can return neg values. For example, (-11 % 5) will return -1,
        // not 4. We actually want the value to be 4 (since we're wrapping around the index).
        (((index % max) + max) % max) as usize
    }

    /// Returns the next index, adjusted for wrap-around
    fn next_index(&self, index: usize) -> usize {
        self.adjust_index(index as isize + 1)
    }

    /// Returns the previous index, adjusted for wrap-around
    fn previous_index(&self, index: usize) -> usize {
        self.adjust_index(index as isize - 1)
    }

    /// Shift items in stack over by one element. If we have  available capacity,
    /// then we'll end up shrinking the stack by one element. If we don't have
    /// available capacity, then we'll need to shift the next stack over too.
    pub fn shift(&mut self, stack_num: usize) {
        println!("/// Shifting {}", stack_num);
        let stack_capacity = self.info[stack_num].capacity;

        // If this stack is at its full capacity, then you need to move the next
        // stack over by one element. This stack * can now claim the freed index. */
        if self.info[stack_num].size >= stack_capacity {
            let next_stack = (stack_num + 1) % self.info.len();
            self.shift(next_stack);
            self.info[stack_num].capacity += 1; // claim index that next stack lost
        }

        // Start from end of capacity
        let mut index = self.info[stack_num].last_capacity_index(|i| self.adjust_index(i as isize));

        // shift elements right
        while self.info[stack_num].is_within_stack_capacity(index, self.values.len()) {
            let prev_index = self.previous_index(index);
            self.values[index] = self.values[prev_index]; // move element
            index = prev_index; // Move pointer left
        }

        // Adjust stack data.
        self.values[self.info[stack_num].start] = 0; // Clear old start

        // Move start right
        self.info[stack_num].start = self.next_index(self.info[stack_num].start);
        self.info[stack_num].capacity -= 1;
    }

    /// Expands a stack by shifting adjacent stacks
    fn expand(&mut self, stack_num: usize) {
        println!("/// Expanding stack {}", stack_num);
        let next_stack = (stack_num + 1) % self.info.len();
        self.shift(next_stack);
        self.info[stack_num].capacity += 1;
    }

    /// Push value onto stack num, shifting/expanding stacks as
    //// necessary. Throws exception if all stacks are full.
    ///
    /// # Arguments
    /// * `stack_num` - Index of the stack to push to
    /// * `value` - Value to push
    ///
    /// # Returns
    /// `Ok(())` on success, `Err(StackError::FullStack)` if all stacks are full
    ///
    /// # Example
    /// ```
    /// let mut stacks = MultiStack::new(3, 5);
    /// stacks.push(0, 42).unwrap(); // Push to first stack
    /// ```
    pub fn push(&mut self, stack_num: usize, value: i32) -> Result<(), StackError> {
        println!("/// Pushing stack {}: {}", stack_num, value);
        if self.all_stacks_are_full() {
            return Err(StackError::FullStack);
        }

        if self.info[stack_num].is_full() {
            self.expand(stack_num);
        }

        // Find the index of the top element in the array + 1,and increment the stack pointer
        self.info[stack_num].size += 1;
        let last_idx = self.info[stack_num].last_element_index(|i| self.adjust_index(i as isize));
        self.values[last_idx] = value;
        Ok(())
    }

    /// Pops a value from the specified stack
    ///
    /// # Arguments
    /// * `stack_num` - Index of the stack to pop from
    ///
    /// # Returns
    /// `Ok(value)` on success, `Err(StackError::EmptyStack)` if stack is empty
    pub fn pop(&mut self, stack_num: usize) -> Result<i32, StackError> {
        println!("/// Popping stack {}", stack_num);
        if self.info[stack_num].is_empty() {
            return Err(StackError::EmptyStack);
        }

        let last_idx = self.info[stack_num].last_element_index(|i| self.adjust_index(i as isize));
        let value = self.values[last_idx];
        self.values[last_idx] = 0; // clear item
        self.info[stack_num].size -= 1;
        Ok(value)
    }

    /// Peeks at the top value of the specified stack without removing it
    pub fn peek(&self, stack_num: usize) -> Result<i32, StackError> {
        if self.info[stack_num].is_empty() {
            return Err(StackError::EmptyStack);
        }
        let last_idx = self.info[stack_num].last_element_index(|i| self.adjust_index(i as isize));
        Ok(self.values[last_idx])
    }

    /// Returns a vector containing all values in the specified stack
    pub fn get_stack_values(&self, stack_num: usize) -> Vec<i32> {
        let stack = &self.info[stack_num];
        let mut items = Vec::with_capacity(stack.size);
        for i in 0..stack.size {
            let idx = self.adjust_index((stack.start + i) as isize);
            items.push(self.values[idx]);
        }
        items
    }
}

impl fmt::Display for MultiStack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, _) in self.info.iter().enumerate() {
            let values = self.get_stack_values(i);
            writeln!(f, "{}: {:?}", i, values)?;
        }
        Ok(())
    }
}

pub fn multi_stack() {
    let mut stacks = MultiStack::new(3, 5);

    // Push elements into stack 0
    stacks.push(0, 10).unwrap();
    stacks.push(0, 20).unwrap();
    stacks.push(0, 30).unwrap();

    // Peek at the top element
    println!("Top of stack 0: {:?}", stacks.peek(0));

    // Pop elements from stack 0
    println!("Popped from stack 0: {:?}", stacks.pop(0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let mut stacks = MultiStack::new(3, 5);

        // Test pushing and popping
        stacks.push(0, 10).unwrap();
        stacks.push(0, 20).unwrap();
        assert_eq!(stacks.pop(0).unwrap(), 20);
        assert_eq!(stacks.pop(0).unwrap(), 10);
        assert!(matches!(stacks.pop(0), Err(StackError::EmptyStack)));

        // Test peeking
        stacks.push(1, 100).unwrap();
        assert_eq!(stacks.peek(1).unwrap(), 100);
        assert_eq!(stacks.pop(1).unwrap(), 100);
    }

    #[test]
    fn test_stack_expansion() {
        let mut stacks = MultiStack::new(3, 2);

        // Fill first stack
        stacks.push(0, 1).unwrap();
        stacks.push(0, 2).unwrap();

        // This push should trigger expansion
        stacks.push(0, 3).unwrap();

        assert_eq!(stacks.get_stack_values(0), vec![1, 2, 3]);
    }

    #[test]
    fn test_full_stack_error() {
        let mut stacks = MultiStack::new(2, 1);
        stacks.push(0, 1).unwrap();
        stacks.push(1, 2).unwrap();
        assert!(matches!(stacks.push(0, 3), Err(StackError::FullStack)));
    }
}
