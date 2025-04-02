### Walkthrough Example

**Initial State** (3 stacks, size=2):
```
Values: [0,0 | 0,0 | 0,0]
Stack 0: start=0, size=0, capacity=2
Stack 1: start=2, size=0, capacity=2 
Stack 2: start=4, size=0, capacity=2
```

**Step 1**: Push 3 items to Stack 0
1. Push 10: `[10,0 | 0,0 | 0,0]` (size=1)
2. Push 20: `[10,20 | 0,0 | 0,0]` (size=2)
3. Push 30 → Needs expansion

**Step 2**: `shift(0)` executes:
1. Finds Stack 0 is full (size=2, capacity=2)
2. Recursively calls `shift(1)` (next stack)
3. Stack 1 shifts its elements right:
   - Original: `[10,20 | 0,0 | 0,0]`
   - After shift: `[10,20 | 0,0 | 0,0]` (Stack 1 was empty)
4. Stack 0 claims freed space (capacity=3)
5. Shifts its own elements:
   - Before: `[10,20,0 | 0,0 | 0]`
   - After: `[0,10,20 | 0,0 | 0]` (start moves right)

**Final State**:
```
Values: [0,10,20 | 0,0 | 0]
Stack 0: start=1, size=3, capacity=3
Stack 1: start=3, size=0, capacity=1 (reduced) 
Stack 2: unchanged
```

### Key Points
1. **Circular Buffer**: `adjust_index` enables wrapping around the array ends
2. **Recursive Shifting**: Full stacks trigger shifts in neighboring stacks
3. **Capacity Management**: Stacks dynamically borrow/give up capacity
4. **O(n) Complexity**: Each shift operation moves multiple elements

This maintains all stacks in a single array while allowing flexible growth - 
a core feature of the multi-stack design.

