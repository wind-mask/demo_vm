use crate::vm::cpu::{CpuCore, Flags};
use crate::vm::memory::heap::HeapObj;
use crate::vm::memory::stack::StackAddr;
use crate::vm::memory::{CodeAddr, Memory};
use demo_isa::isa::{ISAErr, Inst, Reg, RegType};
use enumflags2::{make_bitflags, BitFlags};

pub trait ISARuner {
    fn run_inst(&mut self, inst: Inst, mem: &mut Memory) -> Option<ISAErr>;
    fn get_reg(&self, reg: Reg) -> RegType;
    fn set_reg(&mut self, reg: Reg, val: RegType);
    fn get_pc(&self) -> CodeAddr;
    fn set_pc(&mut self, pc: CodeAddr);
    fn get_bp(&self) -> StackAddr;
    fn set_bp(&mut self, bp: usize);
    fn get_flags(&self) -> BitFlags<Flags>;
    fn set_flags(&mut self, flags: BitFlags<Flags>);
}
impl ISARuner for CpuCore {
    fn run_inst(&mut self, inst: Inst, mem: &mut Memory) -> Option<ISAErr> {
        run(self, inst, mem)
    }
    fn get_reg(&self, reg: Reg) -> RegType {
        self.regs.get_reg(reg)
    }
    fn set_reg(&mut self, reg: Reg, val: RegType) {
        self.regs.set_reg(reg, val);
    }
    fn get_pc(&self) -> CodeAddr {
        self.regs.get_pc()
    }
    fn set_pc(&mut self, pc: CodeAddr) {
        self.regs.set_pc(pc);
    }
    fn get_bp(&self) -> StackAddr {
        self.regs.get_bp()
    }
    fn set_bp(&mut self, bp: StackAddr) {
        self.regs.set_bp(bp);
    }
    fn get_flags(&self) -> BitFlags<Flags> {
        self.flags
    }
    fn set_flags(&mut self, flags: BitFlags<Flags>) {
        self.flags = flags;
    }
}

fn run(core: &mut dyn ISARuner, inst: Inst, memory: &mut Memory) -> Option<ISAErr> {
    match inst {
        Inst::Nop => {}
        Inst::M(reg, i) => core.set_reg(reg, i),
        Inst::Mov(reg1, reg2) => core.set_reg(reg1, core.get_reg(reg2)),
        Inst::Mod(reg1, reg2, reg3) => {
            let r2 = core.get_reg(reg2);
            let r3 = core.get_reg(reg3);
            match (r2, r3) {
                (RegType::Usize(i2), RegType::Usize(i3)) => {
                    if i3 == 0 {
                        return Some(ISAErr::DivByZero);
                    }
                    if let (i, false) = i2.overflowing_rem(i3) {
                        core.set_reg(reg1, RegType::Usize(i))
                    } else {
                        core.set_flags(make_bitflags!(Flags::{Overflow}));
                    }
                }
                _ => return Some(ISAErr::TypeMismatch),
            }
        }
        Inst::AddU(reg1, reg2, reg3) => {
            let r2 = core.get_reg(reg2);
            let r3 = core.get_reg(reg3);
            match (r2, r3) {
                (RegType::Usize(i2), RegType::Usize(i3)) => {
                    if let (i, false) = i2.overflowing_add(i3) {
                        core.set_reg(reg1, RegType::Usize(i))
                    } else {
                        core.set_flags(make_bitflags!(Flags::{Overflow}));
                    }
                }
                _ => return Some(ISAErr::TypeMismatch),
            }
        }
        Inst::AddD(reg1, reg2, reg3) => {
            let r2 = core.get_reg(reg2);
            let r3 = core.get_reg(reg3);
            match (r2, r3) {
                (RegType::F64(f2), RegType::F64(f3)) => core.set_reg(reg1, RegType::F64(f2 + f3)),
                _ => return Some(ISAErr::TypeMismatch),
            }
        }
        Inst::SubU(reg1, reg2, reg3) => {
            let r2 = core.get_reg(reg2);
            let r3 = core.get_reg(reg3);
            match (r2, r3) {
                (RegType::Usize(i2), RegType::Usize(i3)) => {
                    if let (i, false) = i2.overflowing_sub(i3) {
                        core.set_reg(reg1, RegType::Usize(i))
                    } else {
                        core.set_flags(make_bitflags!(Flags::{Overflow}));
                    }
                }
                _ => return Some(ISAErr::TypeMismatch),
            }
        }
        Inst::SubD(reg1, reg2, reg3) => {
            let r2 = core.get_reg(reg2);
            let r3 = core.get_reg(reg3);
            match (r2, r3) {
                (RegType::F64(f2), RegType::F64(f3)) => core.set_reg(reg1, RegType::F64(f2 - f3)),
                _ => return Some(ISAErr::TypeMismatch),
            }
        }
        Inst::MulU(reg1, reg2, reg3) => {
            let r2 = core.get_reg(reg2);
            let r3 = core.get_reg(reg3);
            match (r2, r3) {
                (RegType::Usize(i2), RegType::Usize(i3)) => {
                    if let (i, false) = i2.overflowing_mul(i3) {
                        core.set_reg(reg1, RegType::Usize(i))
                    } else {
                        core.set_flags(make_bitflags!(Flags::{Overflow}));
                    }
                }
                _ => return Some(ISAErr::TypeMismatch),
            }
        }
        Inst::MulD(reg1, reg2, reg3) => {
            let r2 = core.get_reg(reg2);
            let r3 = core.get_reg(reg3);
            match (r2, r3) {
                (RegType::F64(f2), RegType::F64(f3)) => core.set_reg(reg1, RegType::F64(f2 * f3)),
                _ => return Some(ISAErr::TypeMismatch),
            }
        }
        Inst::DivU(reg1, reg2, reg3) => {
            let r2 = core.get_reg(reg2);
            let r3 = core.get_reg(reg3);
            match (r2, r3) {
                (RegType::Usize(i2), RegType::Usize(i3)) => {
                    if i3 == 0 {
                        return Some(ISAErr::DivByZero);
                    }
                    if let (i, false) = i2.overflowing_div(i3) {
                        core.set_reg(reg1, RegType::Usize(i))
                    } else {
                        core.set_flags(make_bitflags!(Flags::{Overflow}));
                    }
                }
                _ => return Some(ISAErr::TypeMismatch),
            }
        }
        Inst::DivD(reg1, reg2, reg3) => {
            let r2 = core.get_reg(reg2);
            let r3 = core.get_reg(reg3);
            match (r2, r3) {
                (RegType::F64(f2), RegType::F64(f3)) => {
                    if f3 == 0.0 {
                        return Some(ISAErr::DivByZero);
                    }
                    core.set_reg(reg1, RegType::F64(f2 / f3))
                }
                _ => return Some(ISAErr::TypeMismatch),
            }
        }
        Inst::And(reg1, reg2, reg3) => {
            let r2 = core.get_reg(reg2);
            let r3 = core.get_reg(reg3);
            match (r2, r3) {
                (RegType::Usize(i2), RegType::Usize(i3)) => {
                    core.set_reg(reg1, RegType::Usize(i2 & i3))
                }
                _ => return Some(ISAErr::TypeMismatch),
            }
        }
        Inst::Or(reg1, reg2, reg3) => {
            let r2 = core.get_reg(reg2);
            let r3 = core.get_reg(reg3);
            match (r2, r3) {
                (RegType::Usize(i2), RegType::Usize(i3)) => {
                    core.set_reg(reg1, RegType::Usize(i2 | i3))
                }
                _ => return Some(ISAErr::TypeMismatch),
            }
        }
        Inst::Xor(reg1, reg2, reg3) => {
            let r2 = core.get_reg(reg2);
            let r3 = core.get_reg(reg3);
            match (r2, r3) {
                (RegType::Usize(i2), RegType::Usize(i3)) => {
                    core.set_reg(reg1, RegType::Usize(i2 ^ i3))
                }
                _ => return Some(ISAErr::TypeMismatch),
            }
        }
        Inst::Not(reg1, reg2) => {
            let r2 = core.get_reg(reg2);
            match r2 {
                RegType::Usize(i2) => core.set_reg(reg1, RegType::Usize(!i2)),
                _ => return Some(ISAErr::TypeMismatch),
            }
        }
        Inst::Neg(reg1, reg2) => {
            let r2 = core.get_reg(reg2);
            match r2 {
                RegType::Usize(i2) => {
                    if let (i, false) = i2.overflowing_neg() {
                        core.set_reg(reg1, RegType::Usize(i))
                    } else {
                        core.set_flags(make_bitflags!(Flags::{Overflow}));
                    }
                }
                RegType::F64(f2) => core.set_reg(reg1, RegType::F64(-f2)),
            }
        }
        Inst::Shl(reg1, reg2) => {
            if let RegType::Usize(i2) = core.get_reg(reg2) {
                if let (i, false) = i2.overflowing_shl(i2 as u32) {
                    core.set_reg(reg1, RegType::Usize(i))
                } else {
                    core.set_flags(make_bitflags!(Flags::{Overflow}));
                }
            } else {
                return Some(ISAErr::TypeMismatch);
            }
        }
        Inst::Shr(reg1, reg2) => {
            if let RegType::Usize(i2) = core.get_reg(reg2) {
                if let (i, false) = i2.overflowing_shr(i2 as u32) {
                    core.set_reg(reg1, RegType::Usize(i))
                } else {
                    core.set_flags(make_bitflags!(Flags::{Overflow}));
                }
            } else {
                return Some(ISAErr::TypeMismatch);
            }
        }
        Inst::LoadH(reg, addr) => match memory.heap_segment.get(addr) {
            Some(HeapObj::R(val)) => core.set_reg(reg, *val),
            None => {
                memory
                    .heap_segment
                    .resize(addr + 1, HeapObj::R(RegType::Usize(0)));
                core.set_reg(reg, RegType::Usize(0));
            }
        },
        Inst::LoadS(reg1, reg2) => {
            if let RegType::Usize(i) = core.get_reg(reg2) {
                let addr = core.get_bp() as StackAddr + i as StackAddr;
                if let Some(val) = memory.stack_segment.get(addr) {
                    core.set_reg(reg1, *val);
                } else {
                    return Some(ISAErr::InvalidStackAddr);
                }
            } else {
                return Some(ISAErr::InvalidReg);
            }
        }
        Inst::StoreS(reg1, reg2) => {
            if let RegType::Usize(i) = core.get_reg(reg2) {
                let addr = core.get_bp() as StackAddr + i as StackAddr;
                if let Some(val) = memory.stack_segment.get_mut(addr) {
                    *val = core.get_reg(reg1);
                } else {
                    return Some(ISAErr::InvalidStackAddr);
                }
            } else {
                return Some(ISAErr::InvalidReg);
            }
        }
        Inst::StoreH(reg, addr) => {
            if let Some(slot) = memory.heap_segment.get_mut(addr) {
                *slot = HeapObj::R(core.get_reg(reg))
            } else {
                memory
                    .heap_segment
                    .resize(addr + 1, HeapObj::R(core.get_reg(reg)));
                memory.heap_segment[addr] = HeapObj::R(core.get_reg(reg));
            }
        }
        Inst::Jo(reg) => {
            if core.get_flags().contains(Flags::Overflow) {
                if let RegType::Usize(addr) = core.get_reg(reg) {
                    core.set_pc(addr as CodeAddr);
                } else {
                    return Some(ISAErr::InvalidReg);
                }
            }
        }
        Inst::Jno(reg) => {
            if !core.get_flags().contains(Flags::Overflow) {
                if let RegType::Usize(addr) = core.get_reg(reg) {
                    core.set_pc(addr as CodeAddr);
                } else {
                    return Some(ISAErr::InvalidReg);
                }
            }
        }
        Inst::Je(reg1, reg2, reg3) => {
            let val1 = core.get_reg(reg1);
            let val2 = core.get_reg(reg2);
            if val1 == val2 {
                if let RegType::Usize(addr) = core.get_reg(reg3) {
                    core.set_pc(addr as CodeAddr);
                } else {
                    return Some(ISAErr::InvalidReg);
                }
            }
        }
        Inst::Jne(reg1, reg2, reg3) => {
            let val1 = core.get_reg(reg1);
            let val2 = core.get_reg(reg2);
            if val1 != val2 {
                if let RegType::Usize(addr) = core.get_reg(reg3) {
                    core.set_pc(addr as CodeAddr);
                } else {
                    return Some(ISAErr::InvalidReg);
                }
            }
        }
        Inst::Jz(reg1, reg2) => {
            let val = core.get_reg(reg1);
            if let RegType::Usize(0) = val {
                let addr = core.get_reg(reg2);
                match addr {
                    RegType::Usize(addr) => core.set_pc(addr as CodeAddr),
                    _ => return Some(ISAErr::InvalidReg),
                }
            }
        }
        Inst::Jnz(reg1, reg2) => {
            let val = core.get_reg(reg1);
            match val {
                RegType::Usize(0) => {}
                _ => {
                    let addr = core.get_reg(reg2);
                    match addr {
                        RegType::Usize(addr) => core.set_pc(addr as CodeAddr),
                        _ => return Some(ISAErr::InvalidReg),
                    }
                }
            }
        }
        Inst::Jmp(reg) => {
            let val = core.get_reg(reg);
            match val {
                RegType::Usize(addr) => core.set_pc(addr as CodeAddr),
                _ => return Some(ISAErr::InvalidReg),
            }
        }
        Inst::Push(reg) => {
            let val = core.get_reg(reg);
            memory.stack_segment.push(val);
        }
        Inst::Pop(reg) => {
            let val = memory.stack_segment.pop();
            match val {
                Some(val) => core.set_reg(reg, val),
                None => return Some(ISAErr::InvalidStackAddr),
            }
        }
        Inst::Call(reg) => {
            if let RegType::Usize(addr) = core.get_reg(reg) {
                memory.stack_segment.push(RegType::Usize(core.get_bp()));
                memory.stack_segment.push(RegType::Usize(core.get_pc()));
                core.set_bp(memory.stack_segment.len() - 1 as StackAddr);
                core.set_pc(addr as CodeAddr);
            } else {
                return Some(ISAErr::InvalidReg);
            }
        }
        Inst::Ret => {
            memory
                .stack_segment
                .resize(core.get_bp() + 1, RegType::Usize(0));
            let pc = memory.stack_segment.pop();
            let bp = memory.stack_segment.pop();
            match (pc, bp) {
                (Some(RegType::Usize(pc)), Some(RegType::Usize(bp))) => {
                    core.set_pc(pc as CodeAddr);
                    core.set_bp(bp as StackAddr);
                }
                _ => return Some(ISAErr::InvalidStackAddr),
            }
        }
        Inst::Halt => return Some(ISAErr::Halt),
    }
    None
}
