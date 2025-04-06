### **Problem: Sort a Stack Using Only Stack Operations**

#### **Objective**:
Sort a stack so that the smallest elements are on top, using only one additional temporary 
stack and no other data structures. You can only use the standard stack operations:
- `push`: Add an element to the top
- `pop`: Remove and return the top element
- `peek`: View the top element without removing it
- `isEmpty`: Check if the stack is empty

---

### **Constraints**:
1. **Only 2 stacks allowed**:  
   - The original stack to be sorted  
   - One temporary stack for intermediate operations  
   - No arrays, lists, or other data structures  

2. **No extra space beyond the two stacks**:  
   - You cannot copy all elements into another structure for sorting.  

3. **Stack operations only**:  
   - No indexing or direct access to elements in the middle of the stack.  

---

### **Approach (Insertion Sort Style)**:
We can sort the stack similarly to how we sort cards in hand (like insertion sort):

1. **Use a Temporary Stack (`sorted`)**:
   - This stack will hold elements in reverse-sorted order (largest at the top).  
   - At the end, we reverse it to get the smallest elements on top.  

2. **Algorithm**:
   - While the original stack is **not empty**:  
     - Pop the top element (`temp`) from the original stack.  
     - While the `sorted` stack is **not empty** and its top element is **greater than `temp`**:  
       - Move elements from `sorted` back to the original stack.  
     - Push `temp` into `sorted`.  
   - Now, `sorted` has elements in descending order (largest on top).  
   - Reverse `sorted` back into the original stack to get ascending order (smallest on top).  

---

### **Example Walkthrough**:
**Original Stack (top to bottom)**: `[5, 2, 7, 1]`  
**Goal**: Sort so that `1` (smallest) is at the top.  

#### **Step 1: Pop from Original, Insert into Sorted Correctly**
1. Pop `5` → `sorted = [5]`  
2. Pop `2` → `5 > 2` → Move `5` back to original → `sorted = [2]`, `original = [5]`  
3. Push `5` → `sorted = [5, 2]`  
4. Pop `7` → `5 < 7` → Push `7` → `sorted = [7, 5, 2]`  
5. Pop `1` → `7 > 1`, `5 > 1`, `2 > 1` → Move all back to original → `sorted = []`, `original = [2, 5, 7]`  
6. Push `1` → `sorted = [1]`  
7. Push `2`, `5`, `7` → `sorted = [7, 5, 2, 1]` (descending order)  

#### **Step 2: Reverse to Get Smallest on Top**
- Pop from `sorted` and push back into `original` → `original = [1, 2, 5, 7]` 
  (sorted, smallest on top).  

---

### **Key Insights**:
1. **Works like Insertion Sort**:  
   - Each element is placed in the correct position in `sorted` by temporarily moving 
     larger elements out of the way.  

2. **Time Complexity**:  
   - **Worst Case**: `O(n²)` (similar to insertion sort for a reversed stack).  
   - **Space Complexity**: `O(n)` (only the two stacks are used).  

3. **Why Two Stacks?**  
   - The temporary stack (`sorted`) keeps partially sorted elements.  
   - The original stack acts as storage while rearranging elements.  

---

### **Alternative Approach (Merge Sort)**:
If recursion is allowed, we can also implement **merge sort**:
1. **Split** the stack into two halves (using a second stack).  
2. **Recursively sort** each half.  
3. **Merge** them back in sorted order.  

But the **insertion-sort style** is more efficient for stack constraints.

---

### **Final Thought**:
This problem tests understanding of **stack operations** and **sorting under constraints**. 
The solution efficiently uses the limited tools (only 2 stacks) to simulate an 
insertion sort. 🚀

