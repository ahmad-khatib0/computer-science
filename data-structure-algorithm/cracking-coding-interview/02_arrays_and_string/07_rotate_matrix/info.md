Break down the logic of the nested `for` loops in the `rotate` function using an example
matrix. The goal of the function is to rotate the matrix **90 degrees clockwise** in place.

### Example Matrix:
Consider the following 3x3 matrix:

```
Original Matrix:
1 2 3
4 5 6
7 8 9
```

### Step-by-Step Explanation:

#### 1. **Outer Loop (`for layer in 0..n / 2`)**:
   - The outer loop iterates over the "layers" of the matrix. For an `n x n` matrix, there 
     are `n / 2` layers.
   - For a 3x3 matrix, `n / 2 = 1`, so there is only **1 layer** to process.
   - The layer is defined by its boundaries:
     - `first = layer` (starting index of the layer).
     - `last = n - 1 - layer` (ending index of the layer).

   For our example:
   - `layer = 0`, `first = 0`, `last = 2`.

#### 2. **Inner Loop (`for i in first..last`)**:
   - The inner loop processes each element in the current layer.
   - For each element at position `(first, i)`, we perform a **four-way swap** to rotate 
     it 90 degrees clockwise.

   For our example:
   - `i` ranges from `0` to `1` (since `first = 0` and `last = 2`).

#### 3. **Four-Way Swap**:
   - For each element in the layer, we perform the following swaps:
     1. Save the top element (`matrix[first][i]`).
     2. Move the left element to the top.
     3. Move the bottom element to the left.
     4. Move the right element to the bottom.
     5. Move the saved top element to the right.

   Let's visualize this for each iteration of the inner loop.

---

### Iteration 1: `i = 0`
#### Initial State:
```
Layer Boundaries:
first = 0, last = 2

Matrix:
1 2 3
4 5 6
7 8 9
```

#### Step 1: Save the top element.
   - `top = matrix[0][0] = 1`.

#### Step 2: Move the left element to the top.
   - `matrix[0][0] = matrix[2][0] = 7`.

   Matrix after this step:
   ```
   7 2 3
   4 5 6
   7 8 9
   ```

#### Step 3: Move the bottom element to the left.
   - `matrix[2][0] = matrix[2][2] = 9`.

   Matrix after this step:
   ```
   7 2 3
   4 5 6
   9 8 9
   ```

#### Step 4: Move the right element to the bottom.
   - `matrix[2][2] = matrix[0][2] = 3`.

   Matrix after this step:
   ```
   7 2 3
   4 5 6
   9 8 3
   ```

#### Step 5: Move the saved top element to the right.
   - `matrix[0][2] = top = 1`.

   Matrix after this step:
   ```
   7 2 1
   4 5 6
   9 8 3
   ```

---

### Iteration 2: `i = 1`
#### Initial State:
```
Matrix:
7 2 1
4 5 6
9 8 3
```

#### Step 1: Save the top element.
   - `top = matrix[0][1] = 2`.

#### Step 2: Move the left element to the top.
   - `matrix[0][1] = matrix[1][0] = 4`.

   Matrix after this step:
   ```
   7 4 1
   4 5 6
   9 8 3
   ```

#### Step 3: Move the bottom element to the left.
   - `matrix[1][0] = matrix[2][1] = 8`.

   Matrix after this step:
   ```
   7 4 1
   8 5 6
   9 8 3
   ```

#### Step 4: Move the right element to the bottom.
   - `matrix[2][1] = matrix[1][2] = 6`.

   Matrix after this step:
   ```
   7 4 1
   8 5 6
   9 6 3
   ```

#### Step 5: Move the saved top element to the right.
   - `matrix[1][2] = top = 2`.

   Matrix after this step:
   ```
   7 4 1
   8 5 2
   9 6 3
   ```

---

### Final Rotated Matrix:
After processing all layers and iterations, the matrix is rotated 90 degrees clockwise:

```
7 4 1
8 5 2
9 6 3
```

### Key Takeaways:
- The outer loop processes each layer of the matrix.
- The inner loop performs a four-way swap for each element in the current layer.
- The four-way swap ensures that each element is moved to its correct position in 
  the rotated matrix.

This logic works for any `n x n` matrix!
