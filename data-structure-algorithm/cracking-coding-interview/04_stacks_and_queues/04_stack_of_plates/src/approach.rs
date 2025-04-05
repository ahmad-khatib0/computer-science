#[derive(Debug)]
struct Node {
    above: Option<Box<Node>>,
    below: Option<Box<Node>>,
    value: i32,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            above: None,
            below: None,
            value,
        }
    }
}

struct Stack {
    capacity: usize,
    top: Option<Box<Node>>,
    bottom: Option<Box<Node>>,
    size: usize,
}

impl Stack {
    fn new(capacity: usize) -> Self {
        Stack {
            capacity,
            top: None,
            bottom: None,
            size: 0,
        }
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    /// Example: When we push 20 onto [10]:
    /// 1. above is the new node (20)
    /// 2. below is the current top (10)
    /// 3. After join:
    ///     10's above points to 20
    ///     20's below points to 10
    fn join(above: &mut Option<Box<Node>>, below: &mut Option<Box<Node>>) {
        if let Some(below_node) = below {
            below_node.above = above.take();
        }

        if let Some(above_node) = above {
            above_node.below = below.take();
        }
    }

    /// Example: Pushing 30 onto [10 <-> 20]:
    /// 1\. Check capacity (OK)
    /// 2. Create node for 30
    /// 3. Take current top (20)
    /// 4. Join 30 and 20:
    ///     . 30's below = 20
    ///     . 20's above = 30
    /// 5. Set 30 as new top
    pub fn push(&mut self, v: i32) -> bool {
        if self.size >= self.capacity {
            return false;
        }

        self.size += 1;
        let mut new_node = Box::new(Node::new(v));

        if let Some(mut old_top) = self.top.take() {
            old_top.above = Some(Box::new(Node::new(v))); // Correct reference management
            new_node.below = Some(old_top);
            self.top = Some(new_node);
        } else {
            self.bottom = Some(new_node);
            self.top = self.bottom.as_ref().map(|b| Box::new(Node::new(b.value)));
        }

        true
    }

    /// Example: Popping from [10 <-> 20 <-> 30]:
    /// 1\. Take top node (30)
    /// 2. Set new top to 20
    /// 3. Clear 20's above pointer (was pointing to 30)
    /// 4. Decrement size (now 2)
    /// 5. Return 30
    /// Stack becomes: [10 <-> 20]
    fn pop(&mut self) -> Option<i32> {
        let mut top = self.top.take()?;
        self.top = top.below.take(); // Set new top to below node

        // If new top exists: clear its above pointer
        if let Some(new_top) = &mut self.top {
            new_top.above = None;
        }

        self.size -= 1;
        if self.size == 0 {
            self.bottom = None;
        }

        Some(top.value)
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Example: Removing bottom from [10 <-> 20 <-> 30]:
    /// 1\. Take bottom node (10)
    /// 2. Set new bottom to 20
    /// 3. Clear 20's below pointer (was pointing to 10)
    /// 4. Decrement size (now 2)
    /// 5. Return 10
    /// Stack becomes: [20 <-> 30]
    fn remove_bottom(&mut self) -> Option<i32> {
        let mut bottom = self.bottom.take()?;
        self.bottom = bottom.above.take();

        // If new bottom exists: Clear its below pointer
        if let Some(new_bottom) = &mut self.bottom {
            new_bottom.below = None;
        }

        self.size -= 1;
        if self.size == 0 {
            self.top = None;
        }
        Some(bottom.value)
    }
}

pub struct SetOfStacks {
    stacks: Vec<Stack>,
    capacity: usize,
}

impl SetOfStacks {
    pub fn new(capacity: usize) -> Self {
        SetOfStacks {
            stacks: Vec::new(),
            capacity,
        }
    }

    fn get_last_stack(&mut self) -> Option<&mut Stack> {
        self.stacks.last_mut()
    }

    pub fn push(&mut self, v: i32) {
        if let Some(last) = self.get_last_stack() {
            if !last.is_full() {
                last.push(v);
                return;
            }
        }

        let mut stack = Stack::new(self.capacity);
        stack.push(v);
        self.stacks.push(stack);
    }

    pub fn pop(&mut self) -> Option<i32> {
        let last = self.stacks.last_mut()?;
        let v = last.pop()?;

        if last.is_empty() {
            self.stacks.pop();
        }

        Some(v)
    }

    pub fn pop_at(&mut self, index: usize) -> Option<i32> {
        self.left_shift(index, true)
    }

    fn left_shift(&mut self, index: usize, remove_top: bool) -> Option<i32> {
        if index >= self.stacks.len() {
            return None;
        }

        let stack = &mut self.stacks[index];
        let removed_item = if remove_top {
            stack.pop()
        } else {
            stack.remove_bottom()
        };

        if stack.is_empty() {
            self.stacks.remove(index);
        } else if self.stacks.len() > index + 1 {
            // When: There exists a next stack (index+1)
            // Action: Roll over the bottom item from the next stack
            //
            // Example: Initial State (capacity=2):
            // Stack 0: [10, 20]
            // Stack 1: [30, 40]
            // Stack 2: [50]
            // Operation: pop_at(1) (remove from Stack 1)
            // 1. Remove 40 from Stack 1 → Stack 1 becomes [30]
            // 2. stack.is_empty() is false
            // 3. Next stack exists (Stack 2)
            // 4. self.left_shift(2, false):
            //     Removes bottom of Stack 2 (50)
            //     Stack 2 becomes empty → gets removed
            // 5. Pushes 50 into Stack 1
            // Final State:
            //    Stack 0: [10, 20]
            //    Stack 1: [30, 50]  // 50 rolled over from Stack 2
            if let Some(v) = self.left_shift(index + 1, false) {
                self.stacks[index].push(v);
            }
        }

        removed_item
    }

    fn is_empty(&self) -> bool {
        self.stacks.is_empty() || self.stacks.last().map_or(true, |s| s.is_empty())
    }
}
