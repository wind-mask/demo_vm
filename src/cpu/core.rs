use crate::cpu::CpuCore;
use crate::memory::Memory;
use crate::sys_call::SYS_CALL_TABLE;

use demo_isa::err::ISAErr;
use demo_isa::reg::{Flags, Reg};
use demo_isa::{CodeAddr, ISARuner, Inst, MemoryRuner, RegType, StackAddr};
use enumflags2::{make_bitflags, BitFlags};

impl ISARuner for CpuCore {
    type M = Memory;
    fn run_inst(&mut self, inst: Inst, mem: &mut Self::M) -> Result<(), ISAErr> {
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
//TODO:寄存器类型优化
pub(crate) fn run(core: &mut CpuCore, inst: Inst, memory: &mut Memory) -> Result<(), ISAErr> {
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
                        return Err(ISAErr::DivByZero);
                    }
                    if let (i, false) = i2.overflowing_rem(i3) {
                        core.set_reg(reg1, RegType::Usize(i))
                    } else {
                        core.set_flags(make_bitflags!(Flags::{Overflow}));
                    }
                }
                _ => return Err(ISAErr::TypeMismatch),
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
                _ => return Err(ISAErr::TypeMismatch),
            }
        }
        Inst::AddD(reg1, reg2, reg3) => {
            let r2 = core.get_reg(reg2);
            let r3 = core.get_reg(reg3);
            match (r2, r3) {
                (RegType::F64(f2), RegType::F64(f3)) => core.set_reg(reg1, RegType::F64(f2 + f3)),
                _ => return Err(ISAErr::TypeMismatch),
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
                _ => return Err(ISAErr::TypeMismatch),
            }
        }
        Inst::SubD(reg1, reg2, reg3) => {
            let r2 = core.get_reg(reg2);
            let r3 = core.get_reg(reg3);
            match (r2, r3) {
                (RegType::F64(f2), RegType::F64(f3)) => core.set_reg(reg1, RegType::F64(f2 - f3)),
                _ => return Err(ISAErr::TypeMismatch),
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
                _ => return Err(ISAErr::TypeMismatch),
            }
        }
        Inst::MulD(reg1, reg2, reg3) => {
            let r2 = core.get_reg(reg2);
            let r3 = core.get_reg(reg3);
            match (r2, r3) {
                (RegType::F64(f2), RegType::F64(f3)) => core.set_reg(reg1, RegType::F64(f2 * f3)),
                _ => return Err(ISAErr::TypeMismatch),
            }
        }
        Inst::DivU(reg1, reg2, reg3) => {
            let r2 = core.get_reg(reg2);
            let r3 = core.get_reg(reg3);
            match (r2, r3) {
                (RegType::Usize(i2), RegType::Usize(i3)) => {
                    if i3 == 0 {
                        return Err(ISAErr::DivByZero);
                    }
                    if let (i, false) = i2.overflowing_div(i3) {
                        core.set_reg(reg1, RegType::Usize(i))
                    } else {
                        core.set_flags(make_bitflags!(Flags::{Overflow}));
                    }
                }
                _ => return Err(ISAErr::TypeMismatch),
            }
        }
        Inst::DivD(reg1, reg2, reg3) => {
            let r2 = core.get_reg(reg2);
            let r3 = core.get_reg(reg3);
            match (r2, r3) {
                (RegType::F64(f2), RegType::F64(f3)) => {
                    if f3 == 0.0 {
                        return Err(ISAErr::DivByZero);
                    }
                    core.set_reg(reg1, RegType::F64(f2 / f3))
                }
                _ => return Err(ISAErr::TypeMismatch),
            }
        }
        Inst::And(reg1, reg2, reg3) => {
            let r2 = core.get_reg(reg2);
            let r3 = core.get_reg(reg3);
            match (r2, r3) {
                (RegType::Usize(i2), RegType::Usize(i3)) => {
                    core.set_reg(reg1, RegType::Usize(i2 & i3))
                }
                _ => return Err(ISAErr::TypeMismatch),
            }
        }
        Inst::Or(reg1, reg2, reg3) => {
            let r2 = core.get_reg(reg2);
            let r3 = core.get_reg(reg3);
            match (r2, r3) {
                (RegType::Usize(i2), RegType::Usize(i3)) => {
                    core.set_reg(reg1, RegType::Usize(i2 | i3))
                }
                _ => return Err(ISAErr::TypeMismatch),
            }
        }
        Inst::Xor(reg1, reg2, reg3) => {
            let r2 = core.get_reg(reg2);
            let r3 = core.get_reg(reg3);
            match (r2, r3) {
                (RegType::Usize(i2), RegType::Usize(i3)) => {
                    core.set_reg(reg1, RegType::Usize(i2 ^ i3))
                }
                _ => return Err(ISAErr::TypeMismatch),
            }
        }
        Inst::Not(reg1, reg2) => {
            let r2 = core.get_reg(reg2);
            match r2 {
                RegType::Usize(i2) => core.set_reg(reg1, RegType::Usize(!i2)),
                _ => return Err(ISAErr::TypeMismatch),
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
                return Err(ISAErr::TypeMismatch);
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
                return Err(ISAErr::TypeMismatch);
            }
        }
        Inst::LoadH(reg_v, reg_a) => {
            if let RegType::Usize(addr) = core.get_reg(reg_a) {
                core.set_reg(reg_v, memory.get_heap(addr));
            } else {
                return Err(ISAErr::InvalidReg);
            }
        }
        Inst::LoadS(reg_v, reg_a) => {
            if let RegType::Usize(addr) = core.get_reg(reg_a) {
                core.set_reg(reg_v, memory.get_stack(core.get_bp(), addr)?);
            } else {
                return Err(ISAErr::InvalidReg);
            }
        }
        Inst::StoreS(reg_v, reg_a) => {
            if let RegType::Usize(addr) = core.get_reg(reg_a) {
                memory.set_stack(core.get_bp(), addr, core.get_reg(reg_v))?;
            } else {
                return Err(ISAErr::InvalidReg);
            }
        }
        Inst::StoreH(reg_v, reg_a) => {
            if let RegType::Usize(addr) = core.get_reg(reg_a) {
                memory.set_heap(addr, core.get_reg(reg_v));
            } else {
                return Err(ISAErr::InvalidReg);
            }
        }
        Inst::Jo(reg) => {
            if core.get_flags().contains(Flags::Overflow) {
                if let RegType::Usize(addr) = core.get_reg(reg) {
                    core.set_pc(addr as CodeAddr);
                } else {
                    return Err(ISAErr::InvalidReg);
                }
            }
        }
        Inst::Jno(reg) => {
            if !core.get_flags().contains(Flags::Overflow) {
                if let RegType::Usize(addr) = core.get_reg(reg) {
                    core.set_pc(addr as CodeAddr);
                } else {
                    return Err(ISAErr::InvalidReg);
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
                    return Err(ISAErr::InvalidReg);
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
                    return Err(ISAErr::InvalidReg);
                }
            }
        }
        Inst::Jz(reg1, reg2) => {
            let val = core.get_reg(reg1);
            if let RegType::Usize(0) = val {
                let addr = core.get_reg(reg2);
                match addr {
                    RegType::Usize(addr) => core.set_pc(addr as CodeAddr),
                    _ => return Err(ISAErr::InvalidReg),
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
                        _ => return Err(ISAErr::InvalidReg),
                    }
                }
            }
        }
        Inst::Jmp(reg) => {
            let val = core.get_reg(reg);
            match val {
                RegType::Usize(addr) => core.set_pc(addr as CodeAddr),
                _ => return Err(ISAErr::InvalidReg),
            }
        }
        Inst::Push(reg) => {
            memory.push_stack(core.get_reg(reg));
        }
        Inst::Pop(reg) => {
            core.set_reg(reg, memory.pop_stack()?);
        }
        Inst::Call(reg) => {
            if let RegType::Usize(addr) = core.get_reg(reg) {
                memory.push_stack(RegType::Usize(core.get_bp()));
                memory.push_stack(RegType::Usize(core.get_pc()));
                core.set_bp(memory.get_stack_top_addr());
                core.set_pc(addr as CodeAddr);
            } else {
                return Err(ISAErr::InvalidReg);
            }
        }
        Inst::Ret => {
            memory.drop_stack_bp(core.get_bp());
            let pc = memory.pop_stack()?;
            let bp = memory.pop_stack()?;
            match (pc, bp) {
                (RegType::Usize(pc), RegType::Usize(bp)) => {
                    core.set_pc(pc as CodeAddr);
                    core.set_bp(bp as StackAddr);
                }
                _ => return Err(ISAErr::TypeMismatch),
            }
        }
        Inst::Halt => return Err(ISAErr::Halt),
        Inst::SysCall(reg) => {
            if let RegType::Usize(sys_call) = core.get_reg(reg) {
                if let Some(sys_call) = SYS_CALL_TABLE.get(sys_call) {
                    sys_call(core, memory)?;
                } else {
                    return Err(ISAErr::InvalidSysCall);
                }
            } else {
                return Err(ISAErr::InvalidReg);
            }
        }
        Inst::In(_, _) => todo!(),  //TODO:实现In
        Inst::Out(_, _) => todo!(), //TODO:实现Out
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
pub struct Regs {
    r1: RegType,
    r2: RegType,
    r3: RegType,
    r4: RegType,
    r5: RegType,
    r6: RegType,
    r7: RegType,
    r8: RegType,
    r9: RegType,
    r10: RegType,
    r11: RegType,
    r12: RegType,
    r13: RegType,
    r14: RegType,
    r15: RegType,
    r16: RegType,
    pc: CodeAddr,
    bp: StackAddr,
}

impl Default for Regs {
    fn default() -> Self {
        Self::new()
    }
}

impl Regs {
    pub fn new() -> Self {
        Regs {
            r1: RegType::Usize(0),
            r2: RegType::Usize(0),
            r3: RegType::Usize(0),
            r4: RegType::Usize(0),
            r5: RegType::Usize(0),
            r6: RegType::Usize(0),
            r7: RegType::Usize(0),
            r8: RegType::Usize(0),
            r9: RegType::Usize(0),
            r10: RegType::Usize(0),
            r11: RegType::Usize(0),
            r12: RegType::Usize(0),
            r13: RegType::Usize(0),
            r14: RegType::Usize(0),
            r15: RegType::Usize(0),
            r16: RegType::Usize(0),

            pc: 0,
            bp: 0,
        }
    }
    pub fn get_reg(&self, reg: Reg) -> RegType {
        match reg {
            Reg::R1 => self.r1,
            Reg::R2 => self.r2,
            Reg::R3 => self.r3,
            Reg::R4 => self.r4,
            Reg::R5 => self.r5,
            Reg::R6 => self.r6,
            Reg::R7 => self.r7,
            Reg::R8 => self.r8,
            Reg::R9 => self.r9,
            Reg::R10 => self.r10,
            Reg::R11 => self.r11,
            Reg::R12 => self.r12,
            Reg::R13 => self.r13,
            Reg::R14 => self.r14,
            Reg::R15 => self.r15,
            Reg::R16 => self.r16,
        }
    }
    pub fn set_reg(&mut self, reg: Reg, val: RegType) {
        match reg {
            Reg::R1 => self.r1 = val,
            Reg::R2 => self.r2 = val,
            Reg::R3 => self.r3 = val,
            Reg::R4 => self.r4 = val,
            Reg::R5 => self.r5 = val,
            Reg::R6 => self.r6 = val,
            Reg::R7 => self.r7 = val,
            Reg::R8 => self.r8 = val,
            Reg::R9 => self.r9 = val,
            Reg::R10 => self.r10 = val,
            Reg::R11 => self.r11 = val,
            Reg::R12 => self.r12 = val,
            Reg::R13 => self.r13 = val,
            Reg::R14 => self.r14 = val,
            Reg::R15 => self.r15 = val,
            Reg::R16 => self.r16 = val,
        }
    }
    pub fn get_bp(&self) -> StackAddr {
        self.bp
    }
    pub fn set_bp(&mut self, bp: StackAddr) {
        self.bp = bp;
    }
    pub fn get_pc(&self) -> CodeAddr {
        self.pc
    }
    pub fn set_pc(&mut self, pc: CodeAddr) {
        self.pc = pc;
    }
}
