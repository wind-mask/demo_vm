extern crate alloc;


fn main() {}

#[cfg(test)]
#[test]
fn test_vm() {
    env_logger::init();
    use demo_isa::{err::ISAErr, Inst::*};
    use demo_vm::{cpu::CpuErr, memory::MemoryErr, VmTmp, VmErr};
    use log::debug;
    let mut v = VmTmp::new();
    let start_code = vec![Nop; 1000];
    println!("{:?}", start_code);
    v.set_code(start_code);

    match v.start() {
        Ok(_) => {}
        Err(e) => match e {
            VmErr::CpuErr(CpuErr::MemoryErr(MemoryErr::InvalidCodeAddr)) => {
                debug!("InvalidCodeAddr");
                debug!("Code {:?}", v.mem_load().0);
            }
            VmErr::CpuErr(CpuErr::ISAErr(err)) => {
                if err == ISAErr::Halt {
                    debug!("Halt");
                    debug!("Memory {:?}", v.mem_load());
                } else {
                    debug!("ISAErr {:?}", err);
                }
            }
            e => {
                debug!("Err {:?}", e);
            }
        },
    }
}
