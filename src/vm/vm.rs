use vm::classfile_parser::attribute_info::code_attribute_parser;
use vm::classfile_parser::ClassFile;
use vm::classfile_parser::constant_info::ConstantInfo;
use vm::classfile_parser::method_info::MethodInfo;
use vm::nom::IResult;
use vm::stack::*;
use vm::class_manager::*;
use std::vec;

pub struct Vm {
    stack: Stack,
    class_manager: ClassManager,
}



impl Vm {
    pub fn run(&mut self) {

    }

    pub fn addClass(&mut self, input_stream: &[u8]){
        self.class_manager.loadClass(input_stream);
    }

    pub fn new() -> Vm {
        return Vm {
            stack: Stack(Vec::new()),
            class_manager: ClassManager::new(),
        }
    }
}