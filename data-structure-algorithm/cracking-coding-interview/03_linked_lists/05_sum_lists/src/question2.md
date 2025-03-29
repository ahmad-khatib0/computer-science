```rust
let val = sum.carry
    + l1.as_ref().map_or(0, |node| node.borrow().data)
    + l2.as_ref().map_or(0, |node| node.borrow().data);
```

### **Understanding the Code**
This snippet computes the sum of two linked lists digit by digit, handling cases where 
a list is shorter than the other. Let's break it down with an example.

---

## **Code Breakdown**
```rust
let val = sum.carry
    + l1.as_ref().map_or(0, |node| node.borrow().data)
    + l2.as_ref().map_or(0, |node| node.borrow().data);
```
- `sum.carry`: Carry from the previous addition step.
- `l1.as_ref().map_or(0, |node| node.borrow().data)`:  
  - If `l1` is `None`, it returns `0`.  
  - Otherwise, it **unwraps** `l1`, accesses the `data` field, and returns its value.
- `l2.as_ref().map_or(0, |node| node.borrow().data)`:  
  - Same as `l1`, but for `l2`.

This ensures that if either `l1` or `l2` is `None`, their contribution to `val` is `0`.

```rust
let full_result = insert_before(sum.sum.take(), val % 10);
```
- **Takes `sum.sum`, inserts the new digit in front, and updates `sum.sum`.**  
- `val % 10`: Extracts the last digit for the current node.

---

## **Example Walkthrough**
Let's add **31** and **591**:
```
l1:  0 -> 3 -> 1
l2:  5 -> 9 -> 1
```

### **Recursive Calls**
| Call | `l1` | `l2` | `sum.carry` | `map_or()` for `l1` | `map_or()` for `l2` | `val = sum.carry + l1 + l2` | `val % 10` | `New carry` |
|------|------|------|------------|------------------|------------------|-----------------|--------|---------|
| **1st Call** | `0` | `5` | `0` | `0` | `5` | `0 + 0 + 5 = 5` | `5` | `0` |
| **2nd Call** | `3` | `9` | `0` | `3` | `9` | `0 + 3 + 9 = 12` | `2` | `1` |
| **3rd Call** | `1` | `1` | `1` | `1` | `1` | `1 + 1 + 1 = 3` | `3` | `0` |
| **Base Case** | `None` | `None` | `0` | `0` | `0` | - | - | - |

#### **Step-by-Step Processing**
1. Base case reached (`None, None`) → Return `PartialSum { sum: None, carry: 0 }`.
2. **Returning from (1,1)**:  
   - `val = 1 + 1 + 1 = 3`
   - `carry = 0`
   - `sum.sum = 3`
3. **Returning from (3,9)**:  
   - `val = 0 + 3 + 9 = 12`
   - `carry = 1`
   - `sum.sum = 2 -> 3`
4. **Returning from (0,5)**:  
   - `val = 1 + 0 + 5 = 6`
   - `carry = 0`
   - `sum.sum = 6 -> 2 -> 3`

---

## **Does `map_or` Iterate Over the List?**
No, **it does not iterate over all items repeatedly**.  

### **How `map_or` Works Per Iteration**
- It is **only applied to the current `l1` and `l2` nodes**.
- **One node is processed per recursive call**.
- It **does not recompute past values** (those are stored in `sum.sum`).

So, **each iteration applies `map_or` only once per `l1` and `l2`, ensuring efficiency.**

---

## **Final Result**
```
  3 -> 1
+ 5 -> 9 -> 1
= 6 -> 2 -> 2
```
Converted back:
```
31 + 591 = 622
```

### **Key Takeaways**
1. **`map_or(0, |node| node.borrow().data)` ensures `None` values are treated as `0`**.
2. **`map_or` does NOT iterate over all elements repeatedly—only the current one**.
3. **Recursive calls move from least significant to most significant digit**.
4. **On the way back up, `insert_before` builds the final sum in order**.


---

## **Code Breakdown**
```rust
let full_result = insert_before(sum.sum.take(), val % 10);
sum.sum = Some(full_result);
sum.carry = val / 10;
sum
```

### **Step-by-Step Explanation**
1. **`val % 10` (Extracts the last digit)**
   - This gives the **current digit** of the sum that should go into the linked list.

2. **`sum.sum.take()` (Takes ownership of the current sum list)**
   - Moves `sum.sum` out of `sum`, replacing it with `None` temporarily.

3. **`insert_before(sum.sum.take(), val % 10)` (Creates new node and inserts before sum)**
   - **Creates a new node** with `val % 10`.
   - The **previous list (sum.sum)** becomes the `.next` of the new node.

4. **`sum.sum = Some(full_result);` (Updates sum with the new node)**
   - Stores the updated linked list in `sum.sum`.

5. **`sum.carry = val / 10;` (Updates carry)**
   - If `val` is 10 or greater, carry is `1` for the next recursion.
   - If `val` is less than 10, carry remains `0`.

6. **`sum` is returned**
   - This ensures that the updated linked list and carry are passed back to the 
     previous recursive call.

---

## **Example Walkthrough**
### **Adding 31 + 591**
```
l1:  0 -> 3 -> 1
l2:  5 -> 9 -> 1
```
---

### **Base Case Returns**
```rust
PartialSum { sum: None, carry: 0 }
```
This is where recursion **starts returning** and begins **building the list**.

---

### **Returning from `add_lists_helper(Some(1), Some(1))`**
```rust
val = 1 + 1 + 0 = 2  // No carry
let full_result = insert_before(None, 2);
```
- **Creates:** `2 -> None`
- **Updates `sum`:**
  ```rust
  sum.sum = Some(2 -> None)
  sum.carry = 0
  ```

---

### **Returning from `add_lists_helper(Some(3), Some(9))`**
```rust
val = 3 + 9 + 0 = 12
let full_result = insert_before(Some(2 -> None), 12 % 10);  // 2
```
- **Creates:** `2 -> 2 -> None`
- **Updates `sum`:**
  ```rust
  sum.sum = Some(2 -> 2 -> None)
  sum.carry = 1
  ```

---

### **Returning from `add_lists_helper(Some(0), Some(5))`**
```rust
val = 0 + 5 + 1 = 6
let full_result = insert_before(Some(2 -> 2 -> None), 6);
```
- **Creates:** `6 -> 2 -> 2 -> None`
- **Updates `sum`:**
  ```rust
  sum.sum = Some(6 -> 2 -> 2 -> None)
  sum.carry = 0
  ```

---

## **Final Output**
```
  3 -> 1
+ 5 -> 9 -> 1
= 6 -> 2 -> 2
```
Converted to integer:
```
31 + 591 = 622
```

---

## **Key Takeaways**
1. **Each recursion returns and inserts a new node at the front.**
2. **`insert_before(sum.sum.take(), val % 10)` ensures the linked list is built from 
   the least significant to the most significant digit.**
3. **The carry (`sum.carry = val / 10`) propagates to the next recursion.**
4. **When recursion fully unwinds, `sum.sum` holds the final linked list representation 
   of the sum.**

---


