pub mod core;


use demo_isa::{err::ISAErr, reg::Flags};
use enumflags2::{make_bitflags, BitFlags};
#[cfg(debug_assertions)]
use log::debug;

use crate::memory::{Memory, MemoryErr};

use self::core::Regs;

#[derive(Debug)]
pub enum CpuErr {
    MemoryErr(MemoryErr),
    ISAErr(ISAErr),
}
impl From<MemoryErr> for CpuErr {
    fn from(err: MemoryErr) -> CpuErr {
        CpuErr::MemoryErr(err)
    }
}
impl From<ISAErr> for CpuErr {
    fn from(err: ISAErr) -> CpuErr {
        CpuErr::ISAErr(err)
    }
}
#[derive(Debug)]
pub struct CpuCore {
    regs: Regs,
    pub flags: BitFlags<Flags>,
}
impl Default for CpuCore {
    fn default() -> Self {
        Self::new()
    }
}

impl CpuCore {
    pub fn new() -> CpuCore {
        let regs = Regs::new();
        CpuCore {
            regs,
            flags: make_bitflags!(Flags::{}),
        }
    }
    pub fn start(&mut self, mem: &mut Memory) -> Result<(), CpuErr> {
        loop {
            let pc = self.regs.get_pc();
            let inst = *mem.fetch_code(pc)?;
            #[cfg(debug_assertions)]
            {
                debug!("pc: {:?}, inst: {:?}", pc, inst);
                debug!("regs: {:?}", self.regs);
                debug!("flags: {:?}", self.flags);
                debug!("stack: {:?}", mem.load().2);
            }
            self.regs.set_pc(pc + 1);
            self.run_inst(&inst, mem)?;
        }
    }
    pub fn reset(&mut self) {
        self.regs.reset();
        self.flags = make_bitflags!(Flags::{}); // clear all flags
        self.set_bp(0);
        self.set_pc(0);
    }
}