// Merge sort implementation for stack
pub fn merge_sort(mut in_stack: Vec<i32>) -> Vec<i32> {
    if in_stack.len() <= 1 {
        return in_stack;
    }

    // Split into left and right
    let mut left = Vec::new();
    let mut right = Vec::new();
    let mut count = 0;
    while let Some(item) = in_stack.pop() {
        if count % 2 == 0 {
            left.push(item);
        } else {
            right.push(item);
        }
        count += 1;
    }

    // Recursively sort
    let left = merge_sort(left);
    let right = merge_sort(right);

    // Merge the two halves
    let mut merged = Vec::new();
    let mut left = left.into_iter().peekable();
    let mut right = right.into_iter().peekable();

    loop {
        match (left.peek(), right.peek()) {
            (Some(&l), Some(&r)) => {
                if l <= r {
                    // Push smaller element first
                    merged.push(left.next().unwrap());
                } else {
                    merged.push(right.next().unwrap());
                }
            }
            (Some(_), None) => merged.push(left.next().unwrap()),
            (None, Some(_)) => merged.push(right.next().unwrap()),
            (None, None) => break,
        }
    }

    merged
}

// Insertion sort implementation for stack
pub fn sort(s: &mut Vec<i32>) {
    let mut r = Vec::new();

    while let Some(tmp) = s.pop() {
        // Insert each element in sorted order into r
        while !r.is_empty() && *r.last().unwrap() > tmp {
            s.push(r.pop().unwrap());
        }
        r.push(tmp);
    }

    // Copy elements back to original stack
    while let Some(item) = r.pop() {
        s.push(item);
    }
}
