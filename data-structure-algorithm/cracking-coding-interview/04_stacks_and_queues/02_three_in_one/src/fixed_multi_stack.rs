use std::fmt;

#[derive(Debug)]
struct FullStackError;

impl fmt::Display for FullStackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stack is full")
    }
}

#[derive(Debug)]
struct EmptyStackError;

impl fmt::Display for EmptyStackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stack is empty")
    }
}

struct FixedMultiStack {
    number_of_stacks: usize,
    stack_capacity: usize,
    values: Vec<i32>,
    sizes: Vec<usize>,
}

impl FixedMultiStack {
    pub fn new(stack_size: usize) -> Self {
        let number_of_stacks = 3;
        Self {
            number_of_stacks,
            stack_capacity: stack_size,
            values: vec![0; stack_size * number_of_stacks],
            sizes: vec![0; number_of_stacks],
        }
    }

    /// Push value onto stack. */
    pub fn push(&mut self, stack_num: usize, value: i32) -> Result<(), FullStackError> {
        if self.is_full(stack_num) {
            return Err(FullStackError);
        }

        self.sizes[stack_num] += 1;
        let top_index = self.index_of_top(stack_num);
        self.values[top_index] = value;

        Ok(())
    }

    /// Pop item from top stack.
    pub fn pop(&mut self, sn: usize) -> Result<i32, EmptyStackError> {
        if self.is_empty(sn) {
            return Err(EmptyStackError);
        }

        let top_index = self.index_of_top(sn);
        let value = self.values[top_index];
        self.values[top_index] = 0; // clear
        self.sizes[sn] -= 1; // shrink
        Ok(value)
    }

    /// Return top element. */
    pub fn peek(&self, stack_num: usize) -> Result<i32, EmptyStackError> {
        if self.is_empty(stack_num) {
            return Err(EmptyStackError);
        }

        Ok(self.values[self.index_of_top(stack_num)])
    }

    /// Return if stack is empty. */
    pub fn is_empty(&self, stack_num: usize) -> bool {
        self.sizes[stack_num] == 0
    }

    /// Return if stack is full. */
    pub fn is_full(&self, stack_num: usize) -> bool {
        self.sizes[stack_num] == self.stack_capacity
    }

    /// Returns index of the top of the stack. */
    fn index_of_top(&self, stack_num: usize) -> usize {
        let offset = stack_num * self.stack_capacity;
        let size = self.sizes[stack_num];
        offset + size - 1
    }

    pub fn get_values(&self) -> &[i32] {
        &self.values
    }

    pub fn get_stack_values(&self, stack_num: usize) -> Vec<i32> {
        let start = stack_num * self.stack_capacity;
        let end = start + self.sizes[stack_num];
        self.values[start..end].to_vec()
    }

    ///* Convert stack to string */
    pub fn stack_to_string(&self, sn: usize) -> String {
        let items = self.get_stack_values(sn);
        format!("{}: {:?}", sn, items)
    }
}

pub fn fixed_multi_stack() {
    let mut stack = FixedMultiStack::new(5);

    // Push elements into stack 0
    stack.push(0, 10).unwrap();
    stack.push(0, 20).unwrap();
    stack.push(0, 30).unwrap();

    // Peek at the top element
    println!("Top of stack 0: {:?}", stack.peek(0));

    // Pop elements from stack 0
    println!("Popped from stack 0: {:?}", stack.pop(0));

    // Print stack contents
    println!("{}", stack.stack_to_string(0));
}
