use std::fmt::{Debug, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
pub enum StackError {
    EmptyStack,
}

struct StackNode<T> {
    data: T,
    next: Option<NonNull<StackNode<T>>>,
}

pub struct MyStack<T> {
    top: Option<NonNull<StackNode<T>>>,
}

impl<T> MyStack<T> {
    pub fn new() -> Self {
        MyStack { top: None }
    }

    pub fn push(&mut self, item: T) {
        let node = Box::new(StackNode {
            data: item,
            next: self.top,
        });

        // Convert Box to raw pointer and wrap in NonNull
        let node_ptr = NonNull::new(Box::into_raw(node));
        self.top = node_ptr;
    }

    pub fn pop(&mut self) -> Result<T, StackError> {
        unsafe {
            self.top.map_or(Err(StackError::EmptyStack), |top_ptr| {
                // Take ownership of the top node
                let top = Box::from_raw(top_ptr.as_ptr());

                // Update top to next node
                self.top = top.next;

                // Return the data
                Ok(top.data)
            })
        }
    }

    pub fn peek(&self) -> Result<&T, StackError> {
        unsafe {
            self.top.map_or(Err(StackError::EmptyStack), |top_ptr| {
                Ok(&(*top_ptr.as_ptr()).data)
            })
        }
    }

    pub fn is_empty(&self) -> bool {
        self.top.is_none()
    }
}

impl<T> Drop for MyStack<T> {
    fn drop(&mut self) {
        // Clean up all nodes when stack is dropped
        while self.pop().is_ok() {}
    }
}

impl<T: Debug> Debug for MyStack<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MyStack").field("top", &self.top).finish()
    }
}

pub fn my_stack() {
    let mut stack = MyStack::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("Top item: {:?}", stack.peek());

    while !stack.is_empty() {
        println!("Popped: {:?}", stack.pop());
    }

    match stack.pop() {
        Ok(val) => println!("Got value: {}", val),
        Err(StackError::EmptyStack) => println!("Stack is empty!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_stack_is_empty() {
        let stack: MyStack<i32> = MyStack::new();
        assert!(stack.is_empty());
    }

    #[test]
    fn test_push_and_peek() {
        let mut stack = MyStack::new();
        stack.push(42);
        assert_eq!(stack.peek().unwrap(), &42);
        assert!(!stack.is_empty());
    }

    #[test]
    fn test_push_and_pop() {
        let mut stack = MyStack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.pop().unwrap(), 2);
        assert_eq!(stack.pop().unwrap(), 1);
        assert!(stack.pop().is_err());
    }

    #[test]
    fn test_peek_empty_stack() {
        let stack: MyStack<String> = MyStack::new();
        assert!(stack.peek().is_err());
    }

    #[test]
    fn test_stack_drop() {
        let mut stack = MyStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        // Stack will be automatically dropped here
    }

    #[test]
    fn test_multiple_operations() {
        let mut stack = MyStack::new();
        assert!(stack.is_empty());

        stack.push(10);
        stack.push(20);

        assert_eq!(*stack.peek().unwrap(), 20);
        assert_eq!(stack.pop().unwrap(), 20);

        stack.push(30);

        assert_eq!(stack.pop().unwrap(), 30);
        assert_eq!(stack.pop().unwrap(), 10);
        assert!(stack.pop().is_err());
        assert!(stack.is_empty());
    }
}
