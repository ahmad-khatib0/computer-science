### Understanding the Two-Stack Queue

Imagine you're at a restaurant with two stacks of plates (let's call them **Stack A** 
and **Stack B**). You want to serve plates to customers in a **First-In-First-Out (FIFO)**
order (like a queue), but you can only take plates from the top of a stack (Last-In-First-Out).
Here's how you'd do it:

---

### Key Operations with Examples

#### 1. **Enqueue (Adding Items)**
- **Action**: Always push onto **Stack A** (the "newest" stack).  
- **Example**:  
  - Add 10: `Stack A` = [10], `Stack B` = []  
  - Add 20: `Stack A` = [20, 10], `Stack B` = []  
  - Add 30: `Stack A` = [30, 20, 10], `Stack B` = []  

#### 2. **Dequeue/Peek (Removing/Viewing Items)**
- **Rule**: Before removing/viewing, if **Stack B** is empty, transfer *all* items from 
  **Stack A** to **Stack B** (reversing their order).  
- **Example**:  
  - Current state: `Stack A` = [30, 20, 10], `Stack B` = []  
  - **First `remove()`**:  
    - Transfer to `Stack B`: `Stack A` = [], `Stack B` = [10, 20, 30]  
    - Pop from `Stack B`: Returns **10** (the oldest item).  
    - New state: `Stack B` = [20, 30]  
  - **Next `peek()`**: Returns **20** (no transfer needed).  
  - **Next `remove()`**: Returns **20** (no transfer).  
  - New state: `Stack B` = [30]  

#### 3. **Why This Works**
- **Stack A** holds new items in reverse order (newest at top).  
- **Stack B** holds old items in correct queue order (oldest at top).  
- Transferring (**`shift_stacks`**) only happens when **Stack B** is empty, ensuring:  
  - Each item is moved *at most twice* (push to A, pop to B).  
  - Amortized O(1) time per operation.  

---

### Example Walkthrough

1. **Enqueue 10, 20, 30**:  
   - `Stack A`: [30, 20, 10]  
   - `Stack B`: []  

2. **First `remove()`**:  
   - Transfer all to `Stack B`: [10, 20, 30]  
   - Pop 10 (correct FIFO order).  

3. **Enqueue 40**:  
   - `Stack A`: [40]  
   - `Stack B`: [20, 30]  

4. **Next `remove()`**:  
   - `Stack B` isn’t empty → pop 20.  

5. **Next `remove()`**:  
   - Pop 30. Now `Stack B` = []  
   - Next call would transfer `Stack A` ([40]) to `Stack B` ([40]).  

---

### Key Insight
- **Stack B** serves as the "ready-to-dequeue" stack.  
- Transferring is lazy (only when needed), minimizing operations.  
- **Visualization**:  
  ```
  Enqueue: [A] grows downward ↓  
  Dequeue: [B] drains upward ↑  
  ```  

This mimics a queue while using only stack operations! 🚀

