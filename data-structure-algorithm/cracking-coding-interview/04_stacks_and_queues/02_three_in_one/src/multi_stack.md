**Shift Function Core:**
```rust
// 6. Start from end of capacity and shift elements right
let mut index = self.info[stack_num].last_capacity_index(adjust);

while self.info[stack_num].is_within_stack_capacity(index, self.values.len()) {
    let prev_index = self.previous_index(index);
    self.values[index] = self.values[prev_index]; // Move element
    index = prev_index; // Move pointer left
}

// 7. Update stack metadata
self.values[self.info[stack_num].start] = 0; // Clear old start
self.info[stack_num].start = self.next_index(self.info[stack_num].start); // Move start right
self.info[stack_num].capacity -= 1; // Reduce capacity
```

### Corrected Example Walkthrough

**Initial State** (3 stacks, each capacity=2):
```
Values: [0,0, 0,0, 0,0]  // [Stack0|Stack1|Stack2]
Stack0: start=0, size=2, capacity=2 (holds [10,20])
Stack1: start=2, size=0, capacity=2
Stack2: start=4, size=0, capacity=2
```

**When pushing 30 to Stack0:**
1. Stack0 is full → calls `shift(0)`
2. `shift(0)` sees Stack0 needs space → recursively calls `shift(1)`
3. `shift(1)` expands into Stack1's space:
   - Before shift: `[10,20, 0,0, 0,0]`
   - After shift:  `[10,20, 0,0, 0,0]` (Stack1 was empty)
   - Stack0's capacity increases to 3

**Now executing lines 6-7 for Stack0:**
1. `last_capacity_index` = adjusted index of `start(0) + capacity(3) - 1 = 2`
2. **While loop**:
   - Iteration 1 (index=2):
     - `prev_index` = adjust_index(2-1) = 1
     - Move `values[1]` (20) to `values[2]` → `[10,20,20,0,0,0]`
   - Iteration 2 (index=1):
     - `prev_index` = 0
     - Move `values[0]` (10) to `values[1]` → `[10,10,20,0,0,0]`
   - Loop ends (index=0 is start)

3. **After loop**:
   - Clear old start: `values[0] = 0` → `[0,10,20,0,0,0]`
   - Move start right: new start = adjust_index(0+1) = 1
   - Final capacity = 3 (from earlier expansion) - 1 = 2

**Now push 30**:
- Stack0 metadata: start=1, size=2, capacity=2 (can take 1 more)
- `values[last_element_index]` = adjust_index(1+2-1) = 2
- But position 2 already has 20! Wait - this reveals an inconsistency.

### Corrected Sequence with 30

**Proper sequence after shift:**
1. After shift: `[0,10,20,0,0,0]` (space created at index 0)
2. Now `push(30)`:
   - Finds last empty spot at `start(1) + size(2)` = index 3? No!
   - Actually uses `last_element_index + 1` = adjust_index(2 + 1) = 3
   - Stores 30 at index 3: `[0,10,20,30,0,0]`
   - Stack0: size=3, capacity=3 (correct)

### Key Insights
1. The shift creates space by moving elements *right*, freeing up the *leftmost* position
2. The new element (`30`) is inserted at `last_element_index + 1`
3. `30` was inserted - it goes after the shifted elements

