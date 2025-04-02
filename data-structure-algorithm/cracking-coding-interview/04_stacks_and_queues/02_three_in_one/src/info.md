### Understanding the Problem

The question asks you to design a way to implement **three separate stacks** using only
**a single array** as the underlying data structure. This is a classic data structure 
design problem that tests your understanding of:

1. **Stack Operations**: The standard stack operations (push, pop, peek, isEmpty) must 
   work for all three stacks.
2. **Memory Efficiency**: How to partition a single array to accommodate multiple stacks.
3. **Boundary Management**: Preventing stack overflow/underflow when the stacks grow/shrink.

### Key Requirements

1. **Single Array Constraint**: All three stacks must share the same array.
2. **Independent Operations**: Each stack should function independently (pushing to Stack 1 
   shouldn't affect Stack 2 or 3).
3. **Fixed or Flexible Size**: Depending on the problem variant:
   - **Fixed Division**: The array is split into three equal/fixed-size partitions.
   - **Flexible Division**: Stacks can grow dynamically until the entire array is full.

### Conceptual Solutions

#### Fixed-Size Approach
- **Partitioning**: Divide the array into three equal segments:
  - Stack 1: `[0, n/3)`
  - Stack 2: `[n/3, 2n/3)`
  - Stack 3: `[2n/3, n)`
- **Pros**: Simple to implement.
- **Cons**: Wastes space if stacks grow unevenly.

#### Flexible-Space Approach
- **Interleaving**: Stacks grow from opposite ends and the middle:
  - Stack 1: Starts at the beginning (`0`), grows rightward.
  - Stack 2: Starts at the end (`n-1`), grows leftward.
  - Stack 3: Starts in the middle, grows flexibly (requires careful collision detection).
- **Pros**: Efficient space utilization.
- **Cons**: Complex to manage overlaps.

### Practical Considerations
- **Stack Pointers**: Maintain pointers/indices for each stack's `top`.
- **Overflow Handling**: What happens when a stack exceeds its allocated space?
- **Error Cases**: Handle pops from empty stacks or pushes to full stacks.

### Example (Non-Code Illustration)
For an array of size `9`:
- **Fixed Division**:
  - Stack 1: Indices `0-2`
  - Stack 2: Indices `3-5`
  - Stack 3: Indices `6-8`
- **Flexible Division**:
  - Stack 1: Grows left to right (`0, 1, 2, ...`).
  - Stack 2: Grows right to left (`8, 7, 6, ...`).
  - Stack 3: Grows from the middle outward (`4, 3 or 5, ...`).

### Why This Question?
- Tests your ability to **partition resources efficiently**.
- Evaluates **trade-offs** between simplicity and flexibility.
- Common in low-level system design where memory is constrained.

This problem is foundational for understanding memory management in multi-stack or 
multi-queue systems.

