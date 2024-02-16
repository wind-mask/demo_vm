pub mod cpu;
pub(crate) mod isa;
pub mod memory;
pub mod reg;

use crate::vm::cpu::{CpuCore, CpuErr};
use crate::vm::memory::Memory;

#[derive(Debug)]
pub struct Vm {
    pub core: CpuCore,
    pub mem: Memory,
}

impl Default for Vm {
    fn default() -> Self {
        Self::new()
    }
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            core: CpuCore::new(),
            mem: Memory::new(),
        }
    }
    pub fn start(&mut self) -> Result<(), CpuErr> {
        self.core.start(&mut self.mem)
    }
}
