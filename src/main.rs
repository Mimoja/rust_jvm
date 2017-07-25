#[macro_use]
extern crate log;
extern crate env_logger;
extern crate classfile_parser;
extern crate nom;

mod vm;

use vm::cpu::Cpu;
use classfile_parser::parser;
use classfile_parser::constant_info::ConstantInfo;
use nom::IResult;

fn main() {
    env_logger::init();
    debug!("Hello, world!");


    let valid_class = include_bytes!("../HelloWorld.class");
    let res = parser::class_parser(valid_class);
    match res {
        IResult::Done(_, c) => {
            println!("Valid class file, version {},{} const_pool({}), this=const[{}], super=const[{}], interfaces({}), fields({}), methods({}), attributes({}), access({:?})", c.major_version, c.minor_version, c.const_pool_size, c.this_class, c.super_class, c.interfaces_count, c.fields_count, c.methods_count, c.attributes_count, c.access_flags);

            let mut code_const_index = 0;

            println!("Constant pool:");
            for (const_index, const_item) in c.const_pool.iter().enumerate() {
                println!("\t[{}] = {:?}", (const_index + 1), const_item);
                match *const_item {
                    ConstantInfo::Utf8(ref c) => {
                        if c.utf8_string == "Code" {
                            code_const_index = (const_index + 1) as u16;
                        }
                    },
                    _ => {},
                }
            }
            println!("Code index = {}", code_const_index);

            println!("Interfaces:");
            let mut interface_index = 0;
            for i in &c.interfaces {
                println!("\t[{}] = const[{}] = {:?}", interface_index, i, c.const_pool[(i-1) as usize]);

                interface_index += 1;
            }
            println!("Fields:");
            let mut field_index = 0;
            for f in &c.fields {
                println!("\t[{}] Name(const[{}] = {:?}) - access({:?})", field_index, f.name_index, c.const_pool[(f.name_index - 1) as usize], f.access_flags);
                field_index += 1;
            }
            println!("Methods:");
            let mut method_index = 0;
            for m in &c.methods {
                println!("\t[{}] Name(const[{}] = {:?}) - access({:?})", method_index, m.name_index, c.const_pool[(m.name_index - 1) as usize], m.access_flags);
                method_index += 1;

                for a in &m.attributes {
                    if a.attribute_name_index == code_const_index {
                        println!("\t\tCode attr found, len = {}", a.attribute_length);
                        let code_result = classfile_parser::attribute_info::code_attribute_parser(&a.info);
                        match code_result {
                            IResult::Done(_, code) => {
                                println!("\t\t\tCode! code_length = {}", code.code_length);
                            },
                            _ => panic!("Not a valid code attr?"),

                        }
                    }
                        else {
                            println!("\t\tAttribute: {:?}", a);
                        }
                }
            }
            let mut known_classes = vec![c];
            let mut cpu = Cpu::new(known_classes);
            cpu.run();
        },
        _ => panic!("Not a class file"),
    };
}