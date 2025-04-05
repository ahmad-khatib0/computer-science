Explain the `SetOfStacks` methods with a clear example, walking through each 
operation step by step. We'll use a capacity of 3 per stack to keep the example manageable.

### Example Setup
Let's create a `SetOfStacks` with capacity 3 and perform these operations:
1. Push 10, 20, 30, 40, 50
2. Pop twice
3. Pop at stack 0 (middle stack)

### Initial State
```
SetOfStacks { capacity: 3, stacks: [] }
```

---

### 1. `push(10)`
**Code Execution**:
```rust
pub fn push(&mut self, v: i32) {
    if let Some(last) = self.get_last_stack() {  // No last stack yet
        if !last.is_full() {
            last.push(v);
            return;
        }
    }
    // Create new stack
    let mut stack = Stack::new(self.capacity);  // New stack capacity=3
    stack.push(v);                              // Stack now [10]
    self.stacks.push(stack);                   // stacks: [ [10] ]
}
```
**Visualization**:
```
Stack 0: [10] (top)
```

---

### 2. `push(20)`, `push(30)`
Same process, adding to last stack until full:
```
Stack 0: [10 <-> 20 <-> 30] (top)
```

---

### 3. `push(40)`
**Code Execution**:
```rust
pub fn push(&mut self, v: i32) {
    if let Some(last) = self.get_last_stack() {  // Last stack is full (size=3)
        if !last.is_full() { ... }              // Skip
    }
    // Create new stack
    let mut stack = Stack::new(self.capacity);  // New stack capacity=3
    stack.push(v);                              // Stack now [40]
    self.stacks.push(stack);                   // stacks: [ [10,20,30], [40] ]
}
```
**Visualization**:
```
Stack 0: [10 <-> 20 <-> 30] 
Stack 1: [40] (top)
```

---

### 4. `push(50)`
Adds to Stack 1:
```
Stack 0: [10 <-> 20 <-> 30]
Stack 1: [40 <-> 50] (top)
```

---

### 5. `pop()`
**Code Execution**:
```rust
pub fn pop(&mut self) -> Option<i32> {
    let last = self.stacks.last_mut()?;  // Get Stack 1
    let v = last.pop()?;                // Returns 50
    
    if last.is_empty() {                // Stack 1 now has [40]
        self.stacks.pop();              // Not empty, skip
    }
    Some(v)
}
```
**Visualization**:
```
Stack 0: [10 <-> 20 <-> 30]
Stack 1: [40] (top)  // 50 removed
```

---

### 6. `pop()` again
Removes 40 from Stack 1. Since it becomes empty, the stack is removed:
```
Stack 0: [10 <-> 20 <-> 30] (top)  // Stack 1 deleted
```

---

### 7. `pop_at(0)` (Remove from middle stack)
**Code Execution**:
```rust
pub fn pop_at(&mut self, index: usize) -> Option<i32> {
    self.left_shift(index, true)
}

fn left_shift(&mut self, index: usize, remove_top: bool) -> Option<i32> {
    // Step 1: Remove from target stack (Stack 0)
    let stack = &mut self.stacks[index];  // Stack 0: [10,20,30]
    let removed = stack.pop();            // Removes 30
    
    // Step 2: If stack not empty (now [10,20]), no shifting needed
    if stack.is_empty() { ... }           // Not empty
    else if self.stacks.len() > index + 1 { ... }  // No next stack
    
    removed
}
```
**Visualization**:
```
Stack 0: [10 <-> 20]  // 30 removed
```

---

### Key Method Behaviors:
1. **`push`**:
   - Always adds to last non-full stack
   - Creates new stack if last is full

2. **`pop`**:
   - Always removes from last stack
   - Removes empty stacks automatically

3. **`left_shift`**:
   - When removing from middle stack (`pop_at`):
     - Removes top item from target stack
     - Only shifts items if there's a next stack
     - In our example, no shift occurred because there was no following stack

This maintains all values while automatically managing the multiple stacks internally.
The `left_shift` operation ensures we can remove from any stack while maintaining the 
capacity constraints.
