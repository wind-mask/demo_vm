mod core;

use crate::memory::Memory;
use core::Regs;
use demo_isa::err::CpuErr;
use demo_isa::reg::Flags;
use demo_isa::{ISARuner, MemoryRuner};
use enumflags2::{make_bitflags, BitFlags};
#[cfg(debug_assertions)]
use log::debug;

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
            let inst = mem.fetch_code(pc)?;
            #[cfg(debug_assertions)]
            {
                debug!("pc: {:?}, inst: {:?}", pc, inst);
                debug!("regs: {:?}", self.regs);
                debug!("flags: {:?}", self.flags);
                debug!("stack: {:?}", mem.load().2);
            }
            self.regs.set_pc(pc + 1);
            self.run_inst(inst, mem)?;
        }
    }
    pub fn reset(&mut self) {
        self.regs.reset();
        self.flags = make_bitflags!(Flags::{}); // clear all flags
        self.set_bp(0);
        self.set_pc(0);
    }
}
