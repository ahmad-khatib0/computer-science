For a string to be a permutation of a palindrome:
1. **All characters must have even counts**, except for **at most one character** 
   (which can have an odd count). This odd-count character would be the middle character 
   in the palindrome.

---

### The Key Insight:
Instead of keeping track of the exact count of each character, we only need to know
whether the count of each character is **even or odd**. This is because:
- If a character appears an **even number of times**, it can be evenly distributed on 
  both sides of the palindrome.
- If a character appears an **odd number of times**, it can only happen once 
  (for the middle character).

This is similar to flipping a light switch:
- If you flip a switch an **even number of times**, the light ends up in its original 
  state (off).
- If you flip it an **odd number of times**, the light ends up in the opposite state (on).

---

### The Algorithm:
We use a **bit vector** (an integer) to represent the parity (even or odd) of each 
character's count. Here's how it works:

1. **Mapping Characters to Bits**:
   - Each character is mapped to a bit position in the integer. For example:
     - `a` → bit 0
     - `b` → bit 1
     - ...
     - `z` → bit 25

2. **Toggling Bits**:
   - As we process each character in the string, we **toggle** the corresponding bit 
     in the integer:
     - If the bit is `0`, set it to `1` (odd count).
     - If the bit is `1`, set it to `0` (even count).

3. **Checking the Bit Vector**:
   - After processing the entire string, the bit vector will represent the parity 
     of all characters.
   - For the string to be a permutation of a palindrome:
     - **All bits must be `0`** (all characters have even counts), **or**
     - **Exactly one bit is `1`** (one character has an odd count).

4. **Efficient Check for a Single Bit**:
   - To check if at most one bit is set to `1`, we use a clever trick:
     - Subtract `1` from the bit vector and perform a bitwise AND with the original bit vector.
     - If the result is `0`, it means there was **at most one bit set to `1`**.

---

### Example:
Let’s take the string `"rats live on no evil star"`:
1. Process each character and toggle the corresponding bit in the bit vector.
2. At the end, the bit vector will have **at most one bit set to `1`** (because the
   string is a permutation of a palindrome).
3. The algorithm confirms this by checking the bit vector.

---

### Why This Works:
- The bit vector efficiently tracks the parity of each character's count.
- The trick of subtracting `1` and ANDing with the original number ensures that only 
  one bit is set to `1` (if any).

---

### Summary:
The algorithm uses a bit vector to efficiently track whether character counts are 
even or odd. By toggling bits and checking the final bit vector, we can determine 
if the string is a permutation of a palindrome. This approach is both elegant and efficient!


--- 

# Bitwise trick
The expression `bit_vector & (bit_vector - 1)` is a **bitwise trick** that clears the 
**lowest set bit** (the rightmost `1`) in `bit_vector`. 

---

### **What Does "Clear the Lowest Set Bit" Mean?**
- In a binary number, the **lowest set bit** is the rightmost `1`.
- "Clearing" it means setting that bit to `0`.

For example:
- If `bit_vector = 0b01011000`, the lowest set bit is at position `3` (counting from `0`).
- After clearing it, `bit_vector` becomes `0b01010000`.

---

### **How Does `bit_vector & (bit_vector - 1)` Work?**

Let’s break it down:

#### 1. **Subtract 1 from `bit_vector` (`bit_vector - 1`)**:
   - Subtracting `1` from a binary number flips all the bits **after** the lowest set 
     bit (including the lowest set bit itself).
   - For example:
     - If `bit_vector = 0b01011000`, then `bit_vector - 1 = 0b01010111`.
     - Notice how the lowest set bit (at position `3`) and all bits to its right are flipped.

#### 2. **Perform a Bitwise AND (`&`) with the Original `bit_vector`**:
   - The bitwise AND operation compares each bit of `bit_vector` with the corresponding bit of `(bit_vector - 1)`.
   - The result is `1` only if both bits are `1`; otherwise, it’s `0`.

   For example:
   ```
   bit_vector      = 0b01011000
   bit_vector - 1  = 0b01010111
   ----------------------------
   AND Result     = 0b01010000
   ```
   - The lowest set bit (at position `3`) is cleared, and all other bits remain unchanged.

---

### **Why Does This Work?**
- Subtracting `1` from `bit_vector` flips all the bits **after** the lowest set bit 
  (including the lowest set bit itself).
- When you perform a bitwise AND between `bit_vector` and `(bit_vector - 1)`, the 
  flipped bits (including the lowest set bit) become `0` in the result.
- All bits **above** the lowest set bit remain unchanged because they are the same 
  in both `bit_vector` and `(bit_vector - 1)`.

---

### **Example Walkthrough**

Let’s take `bit_vector = 0b01011000` and step through the process:

1. **Original `bit_vector`**:
   ```
   bit_vector = 0b01011000
   ```
   - The lowest set bit is at position `3`.

2. **Subtract 1**:
   ```
   bit_vector - 1 = 0b01010111
   ```
   - All bits after (and including) the lowest set bit are flipped.

3. **Bitwise AND**:
   ```
   bit_vector      = 0b01011000
   bit_vector - 1  = 0b01010111
   ----------------------------
   AND Result     = 0b01010000
   ```
   - The lowest set bit (at position `3`) is cleared.

---

### **Why Is This Useful?**
This trick is often used to:
1. **Check if a number is a power of 2**:
   - A power of 2 has exactly one bit set to `1`.
   - If `bit_vector & (bit_vector - 1) == 0`, then `bit_vector` is a power of 2.

2. **Count the number of set bits**:
   - Repeatedly clear the lowest set bit until `bit_vector` becomes `0`. The number 
     of iterations is the number of set bits.

3. **Check if at most one bit is set**:
   - If `bit_vector & (bit_vector - 1) == 0`, then `bit_vector` has **at most one bit set to `1`**.

---

### **Application in the Palindrome Problem**
In the palindrome problem, we use this trick to check if the `bit_vector` has **at most one bit set to `1`**:
```rust
fn check_at_most_one_bit_set(bit_vector: i32) -> bool {
    (bit_vector & (bit_vector - 1)) == 0
}
```
- If `bit_vector & (bit_vector - 1) == 0`, it means:
  - Either all bits are `0` (no characters have odd counts).
  - Or exactly one bit is `1` (one character has an odd count, which is allowed for the 
    middle character in a palindrome).

---

### **Key Takeaway**
The expression `bit_vector & (bit_vector - 1)` is a clever bitwise trick that clears
the lowest set bit in `bit_vector`. It’s efficient and widely used in low-level 
programming for tasks like counting set bits, checking powers of 2, and solving 
problems like the palindrome permutation.

