use primitive_types::U256;

pub struct Stack {
    data: Vec<U256>,
}

impl Stack {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push(&mut self, value: U256) {
        if self.data.len() >= 1024 {
            panic!("Stack overflow");
        }
        self.data.push(value);
    }

    pub fn pop(&mut self) -> U256 {
        self.data.pop().expect("Stack underflow")
    }

    pub fn peek(&self) -> Option<&U256> {
        self.data.last()
    }

    // ✅ ADD THIS
    pub fn len(&self) -> usize {
        self.data.len()
    }

    // ✅ ADD THIS
    pub fn swap(&mut self, i: usize, j: usize) {
        self.data.swap(i, j);
    }
}