#[macro_use]
extern crate log;
extern crate env_logger;

mod vm;

use vm::cpu::Cpu;

fn main() {
    env_logger::init();

    debug!("Hello, world!");
    let mut c = Cpu::new();
    c.run();
}
