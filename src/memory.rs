const UINT64_MAX: u64 = u64::MAX;
const UINT8_MAX: u8 = u8::MAX;

struct EVMMemory<T> {
    memory: Vec<T>,
}

impl<T> EVMMemory<T> {
    pub fn new(memory: usize) -> EVMMemory<T> {
        EVMMemory { memory: Vec::with_capacity(memory) }
    }

    pub fn store(&mut self, offset: u64, value: u64) {
        if offset < 0 || offset > UINT64_MAX.into() {
            panic!("Invalid memory access: offset: {:?} value: {:?}", offset, value);
        }

        if value < 0 || value > UINT8_MAX.into() {
            panic!("Invalid memory access: offset: {:?} value: {:?}", offset, value);
        }

        // Memory expansion
        // if offset >= self.memory.len() {
        //     self.memory.
        // }
    }

    pub fn load(&mut self, offset: usize) -> u64 {
        if offset < 0 {
            panic!("Invalid memory access: offset: {:?}", offset);
        }

        if offset >= self.memory.len() {
            return 0
        }

        return self.memory[offset]
    }
}