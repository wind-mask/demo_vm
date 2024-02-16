use crate::vm::isa::ISARuner;
use crate::vm::reg::Regs;
use crate::vm::Memory;
use demo_isa::isa::ISAErr;
use enumflags2::{bitflags, make_bitflags, BitFlags};
#[cfg(debug_assertions)]
use log::debug;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CpuErr {
    InvalidCodeAddr,
    ISAErr(ISAErr),
}

#[bitflags]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Flags {
    Overflow,
}
#[derive(Debug)]
pub struct CpuCore {
    pub(crate) regs: Regs,
    pub(crate) flags: BitFlags<Flags>,
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
            let inst = mem.fetch_code(pc);
            #[cfg(debug_assertions)]
            {
                debug!("pc: {:?}, inst: {:?}", pc, inst);
                debug!("regs: {:?}", self.regs);
            }
            self.regs.set_pc(pc + 1);
            if let Some(err) = self.run_inst(inst?, mem) {
                return Err(CpuErr::ISAErr(err));
            }
        }
    }
}
