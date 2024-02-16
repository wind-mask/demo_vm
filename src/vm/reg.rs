use crate::vm::memory::stack::StackAddr;
use crate::vm::memory::CodeAddr;
use demo_isa::isa::{Reg, RegType};

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
