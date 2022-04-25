use bytes::{BytesMut, BufMut};

pub mod evmstack;
pub mod memory;


struct Execution {
    code: BytesMut
    stack: EVMStack<T>,
    memory: EVMMemory<T>,
    pc: u8,
    stopped: bool,
}

impl Execution {
    pub fn new(code: BytesMut) -> Execution {

    }

    pub fn stop(&self) {
        self.stopped = true;
    }

    pub fn read_code(&self, num_bytes: BytesMut) -> u64 {
        
    }
}