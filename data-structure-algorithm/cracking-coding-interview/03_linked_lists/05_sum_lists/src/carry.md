The **carry** in this linked list addition algorithm serves the same fundamental 
purpose as in manual decimal addition: it tracks overflow when the sum of two digits 
exceeds 9. Here's why it's essential and why it's only ever 0 or 1:

### Purpose of the Carry
1. **Digit Overflow Handling**:
   - When adding two single-digit numbers (0-9), their sum can range from 0 to 18.
   - The **units digit** becomes part of the result (`sum % 10`).
   - The **tens digit** (either 0 or 1) becomes the carry.

2. **Propagation**:
   - The carry is added to the next significant digit's calculation.
   - This ensures place values are correctly maintained (like "carrying the 1" in manual addition).

### Why Only 0 or 1?
- **Maximum Possible Sum**:
  - Worst case: `9 (digit1) + 9 (digit2) + 1 (previous carry) = 19`
  - The tens place of 19 is **1**, so the new carry can never exceed 1.

- **Mathematical Guarantee**:
  ```text
  Even with maximum inputs:
  carry_start = 1 (worst case from previous step)
  digit1 = 9
  digit2 = 9
  sum = 9 + 9 + 1 = 19 → new carry = 1 (19 / 10 = 1.9 → floor to 1)
  ```

### Concrete Example
**Adding 8 + 7 (with no initial carry):**
1. `8 + 7 = 15`
2. **Result digit**: `5` (15 % 10)
3. **Carry**: `1` (15 ≥ 10)

**Next digit addition includes this carry:**
- If next digits are `2 + 3` with carry `1`:
  `2 + 3 + 1 = 6` → carry becomes `0` (since 6 < 10)

### Visual Representation
| Digit1 | Digit2 | Previous Carry | Sum | Result Digit | New Carry |
|--------|--------|-----------------|-----|--------------|-----------|
| 7      | 5      | 0               | 12  | 2            | 1         |
| 1      | 9      | 1               | 11  | 1            | 1         |
| 6      | 2      | 1               | 9   | 9            | 0         |

### Key Properties
1. **Binary Carry**:
   - The carry is effectively a boolean flag (0 or 1).
   - This simplifies the logic compared to base systems with larger digit ranges.

2. **Recursive Propagation**:
   ```rust
   add_lists_helper(
       l1_next,
       l2_next, 
       if value >= 10 { 1 } else { 0 } // Carry forwarded
   )
   ```

3. **Termination**:
   - The recursion stops only when:
     - Both input lists are exhausted **and**
     - No carry remains (`carry == 0`)

### Why Not Larger Carries?
- In base-10 (decimal) addition, the maximum carry is 1 because:
  ```text
  Max sum per digit pair = 9 + 9 + 1 (carry) = 19 → carry = 1
  ```
- In other bases (e.g., binary), the carry would follow the same logic for that base's 
  maximum digit sum.

### Edge Case Example
**Adding 999 + 1:**
```
Step 1: 9 + 1 = 10 → carry 1, digit 0
Step 2: 9 + 0 + 1 = 10 → carry 1, digit 0
Step 3: 9 + 0 + 1 = 10 → carry 1, digit 0
Step 4: 0 + 0 + 1 = 1 → carry 0, digit 1
Result: 0 → 0 → 0 → 1 (1000)
```

The carry ensures this edge case is handled correctly, extending the result's 
length when needed.
