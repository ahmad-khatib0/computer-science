//
// with approach1 There's just one issue: if we have a large stack, we waste a lot
// of space by keeping track of the min for every single element. Can we do better?
// We can (maybe) do a bit better than approach1 by using an additional stack
// which keeps track of the mins .
//

pub struct StackWithMin {
    main_stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl StackWithMin {
    pub fn new() -> Self {
        StackWithMin {
            main_stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    pub fn push(&mut self, value: i32) {
        if value <= self.min() {
            self.min_stack.push(value);
        }
        self.main_stack.push(value);
    }

    pub fn pop(&mut self) -> Option<i32> {
        let value = self.main_stack.pop()?;
        if value == self.min() {
            self.min_stack.pop();
        }
        Some(value)
    }

    pub fn min(&self) -> i32 {
        if self.min_stack.is_empty() {
            i32::MAX
        } else {
            *self.min_stack.last().unwrap()
        }
    }
}
