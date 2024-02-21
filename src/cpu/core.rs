use crate::cpu::CpuCore;
use crate::memory::Memory;
use crate::sys_call::SYS_CALL_TABLE;

use demo_isa::err::ISAErr;
use demo_isa::reg::{F64Reg, F64RegType, Flags, UsizeReg, UsizeRegType};
use demo_isa::{Inst, RegType};
use enumflags2::{make_bitflags, BitFlags};

impl CpuCore {
    pub fn run_inst(&mut self, inst: &Inst, mem: &mut Memory) -> Result<(), ISAErr> {
        run(self, inst, mem)
    }
    pub fn get_u_reg(&self, ur: UsizeReg) -> UsizeRegType {
        self.regs.get_u_reg(ur)
    }
    fn get_mut_u_reg(&mut self, reg: UsizeReg) -> &mut UsizeRegType {
        self.regs.get_mut_u_reg(reg)
    }
    pub fn get_f_reg(&self, fr: F64Reg) -> F64RegType {
        self.regs.get_f_reg(fr)
    }
    fn get_mut_f_reg(&mut self, reg: F64Reg) -> &mut F64RegType {
        self.regs.get_mut_f_reg(reg)
    }
    pub fn set_u_reg(&mut self, ur: UsizeReg, val: UsizeRegType) {
        self.regs.set_u_reg(ur, val);
    }
    fn set_f_reg(&mut self, fr: F64Reg, val: F64RegType) {
        self.regs.set_f_reg(fr, val);
    }

    pub fn get_pc(&self) -> UsizeRegType {
        self.regs.get_pc()
    }
    pub fn set_pc(&mut self, pc: UsizeRegType) {
        self.regs.set_pc(pc);
    }
    fn get_bp(&self) -> UsizeRegType {
        self.regs.get_bp()
    }
    pub fn set_bp(&mut self, bp: UsizeRegType) {
        self.regs.set_bp(bp);
    }
    fn get_flags(&self) -> BitFlags<Flags> {
        self.flags
    }
    fn set_flags(&mut self, flags: BitFlags<Flags>) {
        self.flags = flags;
    }
}
pub(crate) fn run(core: &mut CpuCore, inst: &Inst, memory: &mut Memory) -> Result<(), ISAErr> {
    match *inst {
        Inst::Nop => {}
        Inst::MU(reg, val) => core.set_u_reg(reg, val),
        Inst::MD(reg, val) => core.set_f_reg(reg, val),
        Inst::MovU(dr, sr) => {
            let val = core.get_u_reg(sr);
            core.set_u_reg(dr, val)
        }
        Inst::MovD(dr, sr) => {
            let v = core.get_f_reg(sr);
            core.set_f_reg(dr, v)
        }
        Inst::Mod(dur, sur1, sur2) => {
            let r1 = core.get_u_reg(sur1);
            let r2 = core.get_u_reg(sur2);
            if r2 == 0 {
                return Err(ISAErr::DivByZero);
            }
            core.set_u_reg(dur, r1 % r2);
        }
        Inst::AddU(dur, sur1, sur2) => {
            if let (i, false) = core.get_u_reg(sur1).overflowing_add(core.get_u_reg(sur2)) {
                core.set_u_reg(dur, i)
            } else {
                core.set_flags(make_bitflags!(Flags::{Overflow}));
            }
        }
        Inst::AddUI(reg, val) => {
            let r = core.get_mut_u_reg(reg);
            if let (i, false) = r.overflowing_add(val) {
                *r = i;
            } else {
                core.set_flags(make_bitflags!(Flags::{Overflow}));
            }
        }
        Inst::AddD(dfr1, sfr1, sfr2) => {
            let r1 = core.get_f_reg(sfr1);
            let r2 = core.get_f_reg(sfr2);
            core.set_f_reg(dfr1, r1 + r2);
        }
        Inst::AddDI(reg, val) => {
            let r = core.get_mut_f_reg(reg);
            *r += val;
        }
        Inst::SubU(dur, sur1, sur2) => {
            let r1 = core.get_u_reg(sur1);
            if let (i, false) = r1.overflowing_sub(core.get_u_reg(sur2)) {
                core.set_u_reg(dur, i)
            } else {
                core.set_flags(make_bitflags!(Flags::{Overflow}));
            }
        }
        Inst::SubUI(reg, val) => {
            let r = core.get_mut_u_reg(reg);
            if let (i, false) = r.overflowing_sub(val) {
                *r = i;
            } else {
                core.set_flags(make_bitflags!(Flags::{Overflow}));
            }
        }
        Inst::SubD(reg1, reg2, reg3) => {
            let r1 = core.get_f_reg(reg2);
            let r2 = core.get_f_reg(reg3);
            core.set_f_reg(reg1, r1 - r2);
        }
        Inst::SubDI(reg, val) => {
            let r = core.get_mut_f_reg(reg);
            *r -= val;
        }
        Inst::MulU(dur, sur1, sur2) => {
            let r1 = core.get_u_reg(sur1);
            let r2 = core.get_u_reg(sur2);
            if let (i, false) = r1.overflowing_mul(r2) {
                core.set_u_reg(dur, i)
            } else {
                core.set_flags(make_bitflags!(Flags::{Overflow}));
            }
        }
        Inst::MulD(dfr, sfr1, sfr2) => {
            let r1 = core.get_f_reg(sfr1);
            let r2 = core.get_f_reg(sfr2);
            core.set_f_reg(dfr, r1 * r2);
        }
        Inst::DivU(dur, sur1, sur2) => {
            let r1 = core.get_u_reg(sur1);
            let r2 = core.get_u_reg(sur2);
            if r2 == 0 {
                return Err(ISAErr::DivByZero);
            }
            core.set_u_reg(dur, r1 / r2);
        }
        Inst::DivD(dfr, sfr1, sfr2) => {
            let r1 = core.get_f_reg(sfr1);
            let r2 = core.get_f_reg(sfr2);
            core.set_f_reg(dfr, r1 / r2);
        }
        Inst::And(dur, sur1, sur2) => {
            let r1 = core.get_u_reg(sur1);
            let r2 = core.get_u_reg(sur2);
            core.set_u_reg(dur, r1 & r2);
        }
        Inst::Or(dur, sur1, sur2) => {
            let r1 = core.get_u_reg(sur1);
            let r2 = core.get_u_reg(sur2);
            core.set_u_reg(dur, r1 | r2);
        }
        Inst::Xor(dur, sur1, sur2) => {
            let r1 = core.get_u_reg(sur1);
            let r2 = core.get_u_reg(sur2);
            core.set_u_reg(dur, r1 ^ r2);
        }
        Inst::Not(dur, sur) => {
            let r = core.get_u_reg(sur);
            core.set_u_reg(dur, !r);
        }
        Inst::NegU(dur, sur) => {
            let r = core.get_u_reg(sur);
            if let (i, false) = r.overflowing_neg() {
                core.set_u_reg(dur, i)
            } else {
                core.set_flags(make_bitflags!(Flags::{Overflow}));
            }
        }
        Inst::NegD(dfr, sfr) => {
            let r = core.get_f_reg(sfr);
            core.set_f_reg(dfr, -r);
        }
        Inst::Shl(dur, sur) => {
            let r = core.get_u_reg(sur);
            if let (i, false) = r.overflowing_shl(1) {
                core.set_u_reg(dur, i)
            } else {
                core.set_flags(make_bitflags!(Flags::{Overflow}));
            }
        }
        Inst::Shr(dur, sur) => {
            let r = core.get_u_reg(sur);
            if let (i, false) = r.overflowing_shr(1) {
                core.set_u_reg(dur, i)
            } else {
                core.set_flags(make_bitflags!(Flags::{Overflow}));
            }
        }
        Inst::LoadUH(reg_v, reg_a) => {
            core.set_u_reg(reg_v, memory.get_heap_u_type(core.get_u_reg(reg_a))?)
        }
        Inst::LoadDH(reg_v, reg_a) => {
            core.set_f_reg(reg_v, memory.get_heap_f_type(core.get_u_reg(reg_a))?)
        }
        Inst::StoreUH(reg_v, reg_a) => {
            memory.set_heap(
                core.get_u_reg(reg_a),
                &RegType::Usize(core.get_u_reg(reg_v)),
            );
        }
        Inst::StoreDH(reg_v, reg_a) => {
            memory.set_heap(core.get_u_reg(reg_a), &RegType::F64(core.get_f_reg(reg_v)));
        }
        Inst::Jo(addr_reg) => {
            if core.get_flags().contains(Flags::Overflow) {
                let v = core.get_u_reg(addr_reg);
                core.set_pc(v)
            }
        }
        Inst::Jno(addr_reg) => {
            if !core.get_flags().contains(Flags::Overflow) {
                let v = core.get_u_reg(addr_reg);
                core.set_pc(v)
            }
        }
        Inst::Je(addr_reg, vreg1, vreg2) => {
            let val1 = core.get_u_reg(vreg1);
            let val2 = core.get_u_reg(vreg2);
            if val1 == val2 {
                let v = core.get_u_reg(addr_reg);
                core.set_pc(v)
            }
        }
        Inst::Jne(addr_reg, vreg1, vreg2) => {
            let val1 = core.get_u_reg(vreg1);
            let val2 = core.get_u_reg(vreg2);
            if val1 != val2 {
                let v = core.get_u_reg(addr_reg);
                core.set_pc(v)
            }
        }
        Inst::Jz(addr_reg, vreg) => {
            let val = core.get_u_reg(vreg);
            if val == 0 {
                let v = core.get_u_reg(addr_reg);
                core.set_pc(v)
            }
        }
        Inst::Jnz(addr_reg, vreg) => {
            let val = core.get_u_reg(vreg);
            if val != 0 {
                let v = core.get_u_reg(addr_reg);
                core.set_pc(v)
            }
        }
        Inst::Jmp(reg) => {
            let v = core.get_u_reg(reg);
            core.set_pc(v);
        }
        Inst::PushU(ureg) => {
            memory.push_stack(RegType::Usize(core.get_u_reg(ureg)));
        }
        Inst::PushD(freg) => {
            memory.push_stack(RegType::F64(core.get_f_reg(freg)));
        }
        Inst::PopU(ureg) => {
            let v = memory.pop_stack()?;
            if let RegType::Usize(v) = v {
                core.set_u_reg(ureg, v);
            } else {
                return Err(ISAErr::TypeMismatch);
            }
        }
        Inst::PopD(freg) => {
            let v = memory.pop_stack()?;
            if let RegType::F64(v) = v {
                core.set_f_reg(freg, v);
            } else {
                return Err(ISAErr::TypeMismatch);
            }
        }
        Inst::Call(ureg) => {
            let addr = core.get_u_reg(ureg);
            memory.push_stack(RegType::Usize(core.get_bp()));
            memory.push_stack(RegType::Usize(core.get_pc()));
            core.set_bp(memory.get_stack_top_addr());
            core.set_pc(addr);
        }
        Inst::Ret => {
            memory.drop_stack_bp(core.get_bp());
            let pc = memory.pop_stack()?;
            let bp = memory.pop_stack()?;
            match (pc, bp) {
                (RegType::Usize(pc), RegType::Usize(bp)) => {
                    core.set_pc(pc);
                    core.set_bp(bp);
                }
                _ => return Err(ISAErr::TypeMismatch),
            }
        }
        Inst::Halt => return Err(ISAErr::Halt),
        Inst::SysCall(ureg) => {
            let sys_call = core.get_u_reg(ureg);
            if let Some(sys_call) = SYS_CALL_TABLE.get(sys_call) {
                sys_call(core, memory)?;
            } else {
                return Err(ISAErr::InvalidSysCall);
            }
        }

        Inst::LoadUS(reg_v, reg_a) => {
            if let RegType::Usize(u) = memory.get_stack(core.get_bp(), core.get_u_reg(reg_a))? {
                core.set_u_reg(reg_v, u);
            } else {
                return Err(ISAErr::TypeMismatch);
            }
        }
        Inst::StoreUS(reg_v, reg_a) => {
            memory.set_stack(
                core.get_bp(),
                core.get_u_reg(reg_a),
                RegType::Usize(core.get_u_reg(reg_v)),
            )?;
        }
        Inst::LoadDS(reg_v, reg_a) => {
            if let RegType::F64(f) = memory.get_stack(core.get_bp(), core.get_u_reg(reg_a))? {
                core.set_f_reg(reg_v, f);
            } else {
                return Err(ISAErr::TypeMismatch);
            }
        }
        Inst::StoreDS(reg_v, reg_a) => {
            memory.set_stack(
                core.get_bp(),
                core.get_u_reg(reg_a),
                RegType::F64(core.get_f_reg(reg_v)),
            )?;
        }
        Inst::InD(_, _) => todo!(),
        Inst::OutD(_, _) => todo!(), //TODO:实现Out
        Inst::InU(_, _) => todo!(),  //TODO:实现In
        Inst::OutU(_, _) => todo!(),
    }
    Ok(())
}

#[derive(Debug)]
pub struct RegsTmp {
    pub u1: UsizeRegType,
    pub u2: UsizeRegType,
    pub u3: UsizeRegType,
    pub u4: UsizeRegType,
    pub u5: UsizeRegType,
    pub u6: UsizeRegType,
    pub u7: UsizeRegType,
    pub u8: UsizeRegType,
    pub f1: F64RegType,
    pub f2: F64RegType,
    pub f3: F64RegType,
    pub f4: F64RegType,
    pub f5: F64RegType,
    pub f6: F64RegType,
    pub f7: F64RegType,
    pub f8: F64RegType,
    pub pc: UsizeRegType,
    pub bp: UsizeRegType,
}

impl Default for RegsTmp {
    fn default() -> Self {
        Self::new()
    }
}

impl RegsTmp {
    pub fn new() -> Self {
        RegsTmp {
            u1: 0,
            u2: 0,
            u3: 0,
            u4: 0,
            u5: 0,
            u6: 0,
            u7: 0,
            u8: 0,
            f1: 0.0,
            f2: 0.0,
            f3: 0.0,
            f4: 0.0,
            f5: 0.0,
            f6: 0.0,
            f7: 0.0,
            f8: 0.0,
            pc: 0,
            bp: 0,
        }
    }
    pub fn get_u_reg(&self, reg: UsizeReg) -> UsizeRegType {
        match reg {
            UsizeReg::U1 => self.u1,
            UsizeReg::U2 => self.u2,
            UsizeReg::U3 => self.u3,
            UsizeReg::U4 => self.u4,
            UsizeReg::U5 => self.u5,
            UsizeReg::U6 => self.u6,
            UsizeReg::U7 => self.u7,
            UsizeReg::U8 => self.u8,
        }
    }
    pub fn get_mut_u_reg(&mut self, reg: UsizeReg) -> &mut UsizeRegType {
        match reg {
            UsizeReg::U1 => &mut self.u1,
            UsizeReg::U2 => &mut self.u2,
            UsizeReg::U3 => &mut self.u3,
            UsizeReg::U4 => &mut self.u4,
            UsizeReg::U5 => &mut self.u5,
            UsizeReg::U6 => &mut self.u6,
            UsizeReg::U7 => &mut self.u7,
            UsizeReg::U8 => &mut self.u8,
        }
    }
    pub fn set_u_reg(&mut self, reg: UsizeReg, val: UsizeRegType) {
        match reg {
            UsizeReg::U1 => self.u1 = val,
            UsizeReg::U2 => self.u2 = val,
            UsizeReg::U3 => self.u3 = val,
            UsizeReg::U4 => self.u4 = val,
            UsizeReg::U5 => self.u5 = val,
            UsizeReg::U6 => self.u6 = val,
            UsizeReg::U7 => self.u7 = val,
            UsizeReg::U8 => self.u8 = val,
        }
    }
    pub fn get_f_reg(&self, reg: F64Reg) -> F64RegType {
        match reg {
            F64Reg::F1 => self.f1,
            F64Reg::F2 => self.f2,
            F64Reg::F3 => self.f3,
            F64Reg::F4 => self.f4,
            F64Reg::F5 => self.f5,
            F64Reg::F6 => self.f6,
            F64Reg::F7 => self.f7,
            F64Reg::F8 => self.f8,
        }
    }
    pub fn get_mut_f_reg(&mut self, reg: F64Reg) -> &mut F64RegType {
        match reg {
            F64Reg::F1 => &mut self.f1,
            F64Reg::F2 => &mut self.f2,
            F64Reg::F3 => &mut self.f3,
            F64Reg::F4 => &mut self.f4,
            F64Reg::F5 => &mut self.f5,
            F64Reg::F6 => &mut self.f6,
            F64Reg::F7 => &mut self.f7,
            F64Reg::F8 => &mut self.f8,
        }
    }
    pub fn set_f_reg(&mut self, reg: F64Reg, val: F64RegType) {
        match reg {
            F64Reg::F1 => self.f1 = val,
            F64Reg::F2 => self.f2 = val,
            F64Reg::F3 => self.f3 = val,
            F64Reg::F4 => self.f4 = val,
            F64Reg::F5 => self.f5 = val,
            F64Reg::F6 => self.f6 = val,
            F64Reg::F7 => self.f7 = val,
            F64Reg::F8 => self.f8 = val,
        }
    }
    pub fn get_bp(&self) -> UsizeRegType {
        self.bp
    }
    pub fn set_bp(&mut self, bp: UsizeRegType) {
        self.bp = bp;
    }
    pub fn get_pc(&self) -> UsizeRegType {
        self.pc
    }
    pub fn set_pc(&mut self, pc: UsizeRegType) {
        self.pc = pc;
    }
    pub fn reset(&mut self) {
        self.u1 = 0;
        self.u2 = 0;
        self.u3 = 0;
        self.u4 = 0;
        self.u5 = 0;
        self.u6 = 0;
        self.u7 = 0;
        self.u8 = 0;
        self.f1 = 0.0;
        self.f2 = 0.0;
        self.f3 = 0.0;
        self.f4 = 0.0;
        self.f5 = 0.0;
        self.f6 = 0.0;
        self.f7 = 0.0;
        self.f8 = 0.0;
        self.pc = 0;
        self.bp = 0;
    }
}

#[derive(Debug)]
pub struct Regs {
    usize_regs: [UsizeRegType; 8],
    f64_regs: [F64RegType; 8],
    pc: UsizeRegType,
    bp: UsizeRegType,
}
impl Default for Regs {
    fn default() -> Self {
        Self::new()
    }
}
impl Regs {
    pub fn new() -> Self {
        Regs {
            usize_regs: [0; 8],
            f64_regs: [0.0; 8],
            pc: 0,
            bp: 0,
        }
    }
    pub fn get_u_reg(&self, reg: UsizeReg) -> UsizeRegType {
        // assert!(self.usize_regs.len()>reg as usize);
        self.usize_regs[reg as usize]
    }
    pub fn get_mut_u_reg(&mut self, reg: UsizeReg) -> &mut UsizeRegType {
        // assert!(self.usize_regs.len()>reg as usize);

        &mut self.usize_regs[reg as usize]
    }
    pub fn set_u_reg(&mut self, reg: UsizeReg, val: UsizeRegType) {
        // assert!(self.usize_regs.len()>reg as usize);
        self.usize_regs[reg as usize] = val;
    }
    pub fn get_f_reg(&self, reg: F64Reg) -> F64RegType {
        // assert!(self.f64_regs.len()>reg as usize);

        self.f64_regs[reg as usize]
    }
    pub fn get_mut_f_reg(&mut self, reg: F64Reg) -> &mut F64RegType {
        // assert!(self.f64_regs.len()>reg as usize);
        &mut self.f64_regs[reg as usize]
    }
    pub fn set_f_reg(&mut self, reg: F64Reg, val: F64RegType) {
        // assert!(self.f64_regs.len()>reg as usize);

        self.f64_regs[reg as usize] = val;
    }
    pub fn get_bp(&self) -> UsizeRegType {
        self.bp
    }
    pub fn set_bp(&mut self, bp: UsizeRegType) {
        self.bp = bp;
    }
    pub fn get_pc(&self) -> UsizeRegType {
        self.pc
    }
    pub fn set_pc(&mut self, pc: UsizeRegType) {
        self.pc = pc;
    }
    pub fn reset(&mut self) {
        self.usize_regs = [0; 8];
        self.f64_regs = [0.0; 8];
        self.pc = 0;
        self.bp = 0;
    }
}
