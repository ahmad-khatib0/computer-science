use std::{
    fmt::{Debug, Formatter, Result as FmtResult},
    ptr::NonNull,
};

// Custom error type for queue operations
#[derive(Debug)]
pub enum QueueError {
    EmptyQueue,
}

#[derive(Debug)]
struct QueueNode<T> {
    data: T,
    next: Option<NonNull<QueueNode<T>>>,
}

// Main queue structure
pub struct MyQueue<T> {
    first: Option<NonNull<QueueNode<T>>>,
    last: Option<NonNull<QueueNode<T>>>,
}

impl<T> MyQueue<T> {
    /// Create a new empty queue
    pub fn new() -> Self {
        MyQueue {
            first: None,
            last: None,
        }
    }

    /// Add an item to the end of the queue
    pub fn add(&mut self, item: T) {
        // Allocate new node on the heap
        let node = Box::new(QueueNode {
            data: item,
            next: None,
        });

        // Convert Box to raw pointer
        let node_ptr = NonNull::new(Box::into_raw(node));

        unsafe {
            if let Some(last) = self.last {
                // link the current last node to the new node
                (*last.as_ptr()).next = node_ptr;
            } else {
                // // Queue was empty, set first pointer
                self.first = node_ptr;
            }
            // Update last pointer to new node
            self.last = node_ptr;
        }
    }

    /// Remove and return the first item from the queue
    pub fn remove(&mut self) -> Result<T, QueueError> {
        unsafe {
            self.first.map_or(Err(QueueError::EmptyQueue), |first_ptr| {
                // Take ownership of the first node
                let first = Box::from_raw(first_ptr.as_ptr());

                // Update first pointer to next node
                self.first = first.next;
                // if queue is now empty, clear last pointer
                if self.first.is_none() {
                    self.last = None;
                }
                Ok(first.data)
            })
        }
    }

    // Peek at the first item without removing it
    pub fn peek(&self) -> Result<&T, QueueError> {
        unsafe {
            self.first.map_or(Err(QueueError::EmptyQueue), |first_ptr| {
                Ok(&(*first_ptr.as_ptr()).data)
            })
        }
    }

    // Check if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.first.is_none()
    }
}

impl<T> Drop for MyQueue<T> {
    fn drop(&mut self) {
        // Clean up all nodes when queue is dropped
        while self.remove().is_ok() {}
    }
}

impl<T: Debug> Debug for MyQueue<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("MyQueue")
            .field("first", &self.first)
            .field("last", &self.last)
            .finish()
    }
}

pub fn my_queue() {
    let mut queue = MyQueue::new();

    queue.add(1);
    queue.add(2);
    queue.add(3);

    println!("First item: {:?}", queue.peek());

    while !queue.is_empty() {
        println!("Removed: {:?}", queue.remove());
    }

    match queue.remove() {
        Ok(val) => println!("Got value: {}", val),
        Err(QueueError::EmptyQueue) => println!("Queue is empty!"),
    }
}

#[cfg(test)]
mod test {
    use rand::Rng;

    use super::*;
    use std::collections::VecDeque;

    #[test]
    fn test_queue() {
        // Generate random array of 100 numbers between -100 and 100
        let array: Vec<i32> = (0..100)
            .map(|_| rand::rng().random_range(-100..=100))
            .collect();

        let mut queue1 = MyQueue::new();
        let mut queue2 = VecDeque::new();

        for &a in &array {
            if a < 0 {
                // Remove elements
                let top1 = queue1.remove().unwrap_or(i32::MIN);
                let top2 = queue2.pop_front().unwrap_or(i32::MIN);

                if top1 != top2 {
                    println!("ERROR: mismatching tails");
                } else {
                    println!("SUCCESS: matching tails: {}", top1);
                }
            } else {
                // Add elements
                queue1.add(a);
                queue2.push_back(a);
            }
        }

        // Drain remaining elements
        while !queue1.is_empty() || !queue2.is_empty() {
            let top1 = queue1.remove().unwrap_or(i32::MIN);
            let top2 = queue2.pop_front().unwrap_or(i32::MIN);

            if top1 != top2 {
                println!("ERROR: mismatching tails");
            } else {
                println!("SUCCESS: matching tails: {}", top1);
            }
        }
    }
}
