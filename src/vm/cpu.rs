use vm::stack::*;

pub struct Cpu {
    stack: Stack,
}

impl Cpu {
    pub fn run(&mut self) {
        self.stack.push_ref(16);
    }
    pub fn new() -> Cpu {
        Cpu {
            stack: Stack(Vec::new()),
        }
    }
}