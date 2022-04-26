const UINT64_MAX: u64 = u64::MAX;
const UINT8_MAX: u8 = u8::MAX;

#[derive(Debug)]
pub struct EVMMemory {
    memory: Vec<u8>,
}

impl EVMMemory {
    pub fn new(memory: usize) -> Self {
        Self { memory: Vec::with_capacity(memory) }
    }

    pub fn load(&mut self, offset: usize) -> u8 {
        if offset >= self.memory.len() {
            return 0
        }

        return self.memory[offset]
    }

    // pub fn store(&mut self, offset: usize) -> 
}