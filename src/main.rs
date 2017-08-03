#[macro_use]
extern crate log;
extern crate env_logger;

mod vm;

use vm::vm::Vm;

fn main() {
    env_logger::init();

    let classStream = include_bytes!("../test.class");
    let mut vm = Vm::new();
    vm.run();
}