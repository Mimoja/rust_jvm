use vm::stack::*;
use classfile_parser::attribute_info::code_attribute_parser;
use classfile_parser::ClassFile;
use classfile_parser::constant_info::ConstantInfo;
use classfile_parser::method_info::MethodInfo;
use nom::IResult;
use std::vec;

pub struct Cpu {
    stack: Stack,
    known_classes: Vec<ClassFile>,
}
fn run_method(c: &ClassFile, m: &MethodInfo){

    let mut code_const_index = 0;
    for (const_index, const_item) in c.const_pool.iter().enumerate() {
        match *const_item {
            ConstantInfo::Utf8(ref c) => {
                if c.utf8_string == "Code" {
                    code_const_index = (const_index + 1) as u16;
                }
            },
            _ => {},
        }
    }

    for a in &m.attributes {
        if a.attribute_name_index == code_const_index {
            let code_result = code_attribute_parser(&a.info);
            match code_result {
                IResult::Done(_, code) => {
                    println!("Running opcodes!");
                },
                _ => panic!("Not a valid code attr?"),

            }
        }
    }
}




impl Cpu {
    pub fn run(&mut self) {
        for c in &self.known_classes {
            for m in &c.methods {
                if let Some(&ConstantInfo::Utf8(ref string)) = c.const_pool.get((m.name_index - 1) as usize){
                    if string.utf8_string == "<init>"{
                        println!("found init!");
                        run_method(c,m);
                    }
                    if string.utf8_string == "main"{
                        println!("found main!");
                        run_method(c,m);
                    }
                }
            }
        }
    }
    pub fn new(c: Vec<ClassFile>) -> Cpu {
        Cpu {
            stack: Stack(Vec::new()),
            known_classes: c,
        }
    }
}