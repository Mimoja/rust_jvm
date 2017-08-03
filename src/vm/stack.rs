use vm::variables::Variable;

#[derive(Clone)]
pub enum StackEntry {
    Reference(u32),
    Immediate(Variable),
}

#[derive(Clone)]
pub struct Stack(pub Vec<StackEntry>);

impl Stack {
    pub fn push_ref(&mut self, r: u32) {
        self.0.push(StackEntry::Reference(r));
    }

    pub fn push_imm(&mut self, i: Variable) {
        self.0.push(StackEntry::Immediate(i));
    }

    pub fn pop_ref(&mut self) -> u32 {
        if let Some(entry) = self.0.pop() {
            match entry {
                StackEntry::Reference(val) => val,
                StackEntry::Immediate(_) => panic!("Poping an immediate as Reference"),
            }
        } else {
            panic!("Empty Stack!");
        }
    }

    pub fn pop_imm(&mut self) -> Variable {
        if let Some(entry) = self.0.pop() {
            match entry {
                StackEntry::Reference(_) => panic!("Poping an reference as immediate"),
                StackEntry::Immediate(val) => val,
            }
        } else {
            panic!("Empty Stack!");
        }
    }
}
