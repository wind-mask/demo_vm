extern crate alloc;

fn main() {}

#[cfg(test)]
#[test]
fn test_vm() {
    env_logger::init();
    use demo_isa::reg::Reg;
    use demo_isa::RegType;
    use demo_isa::{
        err::{CpuErr, ISAErr},
        Inst::*,
    };
    use demo_vm::Vm;
    let mut v = Vm::new();
    let mut start_code = vec![M(Reg::R1, RegType::Usize(4)), Call(Reg::R1)];
    let mut code_func = vec![M(Reg::R1, RegType::Usize(2952)), Ret];
    let mut code = vec![
        M(Reg::R1, RegType::Usize(2)),
        Call(demo_isa::reg::Reg::R1),
        Halt,
    ];
    start_code.append(&mut code_func);
    start_code.append(&mut code);
    println!("{:?}", start_code);
    v.push_code(start_code);

    match v.start() {
        Ok(_) => {}
        Err(e) => match e {
            CpuErr::InvalidCodeAddr => {
                println!("InvalidCodeAddr");
                println!("Code {:?}", v.mem_load().0);
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
