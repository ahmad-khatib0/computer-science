mod approach;

fn main() {}

#[cfg(test)]
mod tests {
    use rand::Rng;
    use std::collections::VecDeque;

    use crate::approach::MyQueue;

    #[test]
    fn test_queue_operations() {
        let mut rng = rand::rng();
        let mut my_queue = MyQueue::new();
        let mut reference_queue = VecDeque::new();

        for _ in 0..100 {
            let choice = rng.random_range(0..=10);

            if choice <= 5 {
                // Enqueue test
                let element = rng.random_range(1..=10);
                reference_queue.push_back(element);
                my_queue.add(element);
                println!("[TEST] Enqueued {}", element);
            } else if !reference_queue.is_empty() {
                // Dequeue test
                let expected = reference_queue.pop_front().unwrap();
                let actual = my_queue.remove().unwrap();

                assert_eq!(
                    expected, actual,
                    "Dequeue mismatch: expected {}, got {}",
                    expected, actual
                );
                println!("[TEST] Dequeued {}", actual);

                // Check peek after removal
                let reference_front = reference_queue.front().copied();
                let my_front = my_queue.peek().copied();
                assert_eq!(
                    reference_front, my_front,
                    "Peek mismatch after removal: reference={:?}, my_queue={:?}",
                    reference_front, my_front
                );
            }

            // Check sizes without mutable borrow
            assert_eq!(
                reference_queue.len(),
                my_queue.size(),
                "Queue size mismatch: reference={}, my_queue={}",
                reference_queue.len(),
                my_queue.size()
            );
        }
    }

    #[test]
    fn test_edge_cases() {
        let mut q = MyQueue::new();

        // Test empty queue
        assert!(q.peek().is_none());
        assert!(q.remove().is_none());

        // Single element
        q.add(42);
        assert_eq!(q.peek(), Some(&42));
        assert_eq!(q.remove(), Some(42));
        assert!(q.peek().is_none());

        // Multiple operations
        q.add(1);
        q.add(2);
        assert_eq!(q.remove(), Some(1));
        q.add(3);
        assert_eq!(q.remove(), Some(2));
        assert_eq!(q.remove(), Some(3));
    }
}
