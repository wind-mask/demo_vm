//! 测试的模块，编译去除
use criterion::black_box;
use demo_isa::{
    err::ISAErr,
    reg::{F64Reg, F64RegType, UsizeReg, UsizeRegType},
    Inst,
};
use log::debug;

use crate::{cpu::CpuErr, memory::MemoryErr, VmTmp, VmErr};

impl VmTmp {
    pub fn get_pc(&self) -> UsizeRegType {
        self.core.get_pc()
    }
    pub fn get_u_reg(&self, reg: UsizeReg) -> UsizeRegType {
        self.core.get_u_reg(reg)
    }
    pub fn get_f_reg(&self, reg: F64Reg) -> F64RegType {
        self.core.get_f_reg(reg)
    }
}
pub fn vm_fibonacci(n: usize) -> usize {
    let mut vm = VmTmp::new();
    let code = vec![
        Inst::MU(UsizeReg::U8, 4),
        Inst::MU(UsizeReg::U1, n),
        Inst::Call(UsizeReg::U8), //U1=fib(n)
        Inst::Halt,
        Inst::MU(UsizeReg::U2, 7),             // 4:fib(U1)->U1
        Inst::Jnz(UsizeReg::U2, UsizeReg::U1), // if n==0:return 0
        Inst::Ret,
        Inst::MU(UsizeReg::U2, 12),            //7
        Inst::SubUI(UsizeReg::U1, 1),          //U1=U1-1=n-1
        Inst::Jnz(UsizeReg::U2, UsizeReg::U1), // if n-1==0:return 1
        Inst::MU(UsizeReg::U1, 1),
        Inst::Ret,                 //11
        Inst::PushU(UsizeReg::U1), // push U1=n-1
        Inst::MU(UsizeReg::U2, 4), // U2= fib
        Inst::Call(UsizeReg::U2),  //U1= fib(n-1)
        Inst::PopU(UsizeReg::U2),  // U2=n-1
        Inst::PushU(UsizeReg::U1), //push U1=fib(n-1)
        Inst::MU(UsizeReg::U1, 1),
        Inst::SubU(UsizeReg::U1, UsizeReg::U2, UsizeReg::U1), //U1=n-2
        Inst::MU(UsizeReg::U2, 4),                            // U2= fib
        Inst::Call(UsizeReg::U2),                             // U1=fib(n-2)
        Inst::PopU(UsizeReg::U2),                             // U2=fib(n-1)
        Inst::AddU(UsizeReg::U1, UsizeReg::U2, UsizeReg::U1), //U1=fib(n-1)+fib(n-2)
        Inst::Ret,
    ];
    vm.set_code(code);
    match vm.start() {
        Ok(_) => {}
        Err(VmErr::CpuErr(CpuErr::ISAErr(ISAErr::Halt))) => {
            debug!("Halt")
        }
        Err(VmErr::CpuErr(CpuErr::MemoryErr(MemoryErr::InvalidCodeAddr))) => {
            panic!("InvalidCodeAddr")
        }
        Err(e) => {
            panic!("ISAErr: {:?}", e)
        }
    }
    vm.get_u_reg(demo_isa::reg::UsizeReg::U1)
}
#[cfg(test)]
#[test]
pub fn test_vm_fibonacci() {
    env_logger::init();
    for i in 0..10 {
        assert_eq!(vm_fibonacci(i), fibonacci(i));
    }
}
pub fn fibonacci(n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    black_box(fibonacci(n - 1)) + black_box(fibonacci(n - 2))
}
