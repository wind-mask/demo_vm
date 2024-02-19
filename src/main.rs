extern crate alloc;

fn main() {}

#[cfg(test)]
#[test]
fn test_vm() {
    env_logger::init();
    use demo_isa::{
        err::{CpuErr, ISAErr},
        Inst::*,
    };
    use demo_vm::Vm;
    use log::debug;
    let mut v = Vm::new();
    let start_code = vec![Nop; 1000];
    println!("{:?}", start_code);
    v.set_code(start_code);

    match v.start() {
        Ok(_) => {}
        Err(e) => match e {
            CpuErr::InvalidCodeAddr => {
                debug!("InvalidCodeAddr");
                debug!("Code {:?}", v.mem_load().0);
            }
            CpuErr::ISAErr(err) => {
                if err == ISAErr::Halt {
                    debug!("Halt");
                    debug!("Memory {:?}", v.mem_load());
                } else {
                    debug!("ISAErr {:?}", err);
                }
            }
        },
    }
}
