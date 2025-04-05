use rand::seq::IndexedRandom;

pub struct MyQueue<T> {
    stack_newest: Vec<T>,
    stack_oldest: Vec<T>,
}

impl<T> MyQueue<T> {
    pub fn new() -> Self {
        MyQueue {
            stack_newest: Vec::new(),
            stack_oldest: Vec::new(),
        }
    }

    pub fn size(&self) -> usize {
        self.stack_newest.len() + self.stack_oldest.len()
    }

    pub fn add(&mut self, value: T) {
        self.stack_newest.push(value);
    }

    fn shift_stacks(&mut self) {
        if self.stack_oldest.is_empty() {
            while let Some(value) = self.stack_newest.pop() {
                self.stack_oldest.push(value);
            }
        }
    }

    pub fn peek(&mut self) -> Option<&T> {
        self.shift_stacks();
        self.stack_oldest.last()
    }

    pub fn remove(&mut self) -> Option<T> {
        self.shift_stacks();
        self.stack_oldest.pop()
    }
}
