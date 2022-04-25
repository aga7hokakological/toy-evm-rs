const STACK_SIZE: i32 = 1024;

struct EVMStack<T> {
    max_depth: usize,
    items: Vec<T>,
    top: usize,
}

impl<T> EVMStack<T> {
    pub fn new(max_depth: usize) -> EVMStack<T> {
        EVMStack {
            max_depth: max_depth,
            items: Vec::with_capacity(max_depth),
            top: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        if self.top == self.max_depth {
            panic!("max depth exceeded")
        }

        self.items.push(item);
        self.top += 1
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            panic!("stack underflow")
        }

        self.top -= 1;
        self.items.pop()
    }
}


