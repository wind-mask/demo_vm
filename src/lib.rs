use crate::cpu::CpuCore;
use crate::memory::Memory;
use demo_isa::err::CpuErr;
use demo_isa::{Inst, RegType};
use memory::Stack;
use memory::{Heap, HeapObj};

pub mod cpu;
pub mod memory;
pub mod sys_call;

#[derive(Debug)]
pub struct Vm {
    core: CpuCore,
    mem: Memory,
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
    pub fn push_code(&mut self, code: Vec<Inst>) {
        self.mem.store(Some(code), None, None);
    }
    pub fn mem_store(&mut self, code: Option<Vec<Inst>>, heap: Option<Heap>, stack: Option<Stack>) {
        self.mem.store(code, heap, stack);
    }
    pub fn mem_load(&self) -> (Vec<Inst>, Vec<HeapObj>, Vec<RegType>) {
        self.mem.load()
    }
}
