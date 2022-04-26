const STACK_SIZE: u32 = 1024;

#[derive(Debug)]
pub struct EVMStack<T> {
    max_depth: u32,
    items: Vec<T>,
    top: usize,
}

impl<T> EVMStack<T> {
    pub fn new() -> EVMStack<T> {
        EVMStack {
            max_depth: STACK_SIZE,
            items: Vec::with_capacity(STACK_SIZE.try_into().unwrap()),
            top: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        if self.top == self.max_depth.try_into().unwrap() {
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


