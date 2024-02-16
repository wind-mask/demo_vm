use demo_isa::isa::ISAErr;
use demo_isa::isa::Inst::Nop;
use demo_vm::vm::cpu::CpuErr;
use demo_vm::vm::Vm;
extern crate alloc;

fn main() {
    env_logger::init();
    println!("Hello, world!");
    let mut v = Vm::new();
    let code = vec![Nop];

    v.mem.push_code_vec(code);

    match v.start() {
        Ok(_) => {}
        Err(e) => match e {
            CpuErr::InvalidCodeAddr => {
                println!("InvalidCodeAddr");
            }
            CpuErr::ISAErr(err) => {
                if err == ISAErr::Halt {
                    println!("Halt");
                    println!("Memory {:?}", v.mem.load());
                } else {
                    println!("ISAErr {:?}", err);
                }
            }
        },
    }
}
