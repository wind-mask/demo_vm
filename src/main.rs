use demo_isa::err::{CpuErr, ISAErr};
use demo_isa::Inst::Nop;
use demo_vm::Vm;
extern crate alloc;

fn main() {
    env_logger::init();
    println!("Hello, world!");
    let mut v = Vm::new();
    let code = vec![Nop];

    v.push_code(code);

    match v.start() {
        Ok(_) => {}
        Err(e) => match e {
            CpuErr::InvalidCodeAddr => {
                println!("InvalidCodeAddr");
            }
            CpuErr::ISAErr(err) => {
                if err == ISAErr::Halt {
                    println!("Halt");
                    println!("Memory {:?}", v.mem_load());
                } else {
                    println!("ISAErr {:?}", err);
                }
            }
        },
    }
}
