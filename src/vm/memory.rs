pub mod heap;
pub mod stack;

use crate::vm::cpu::CpuErr;
use crate::vm::memory::heap::{Heap, HeapObj};
use demo_isa::isa::Inst;
use demo_isa::isa::RegType;

pub type CodeAddr = usize;
#[derive(Debug)]
pub struct Memory {
    code_segment: Vec<Inst>,
    pub heap_segment: Heap,
    pub stack_segment: Vec<RegType>,
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    pub fn fetch_code(&self, addr: CodeAddr) -> Result<Inst, CpuErr> {
        if addr < self.code_segment.len() {
            Ok(self.code_segment[addr])
        } else {
            Err(CpuErr::InvalidCodeAddr)
        }
    }
    pub fn new() -> Memory {
        Memory {
            code_segment: Vec::new(),
            heap_segment: Vec::new(),
            stack_segment: Vec::new(),
        }
    }
    pub fn store(
        &mut self,
        code: Option<Vec<Inst>>,
        heap: Option<Vec<HeapObj>>,
        stack: Option<Vec<RegType>>,
    ) {
        if let Some(c) = code {
            self.code_segment = c;
        }
        if let Some(h) = heap {
            self.heap_segment = h;
        }
        if let Some(s) = stack {
            self.stack_segment = s;
        }
    }
    pub fn load(&self) -> (Vec<Inst>, Vec<HeapObj>, Vec<RegType>) {
        (
            self.code_segment.clone(),
            self.heap_segment.clone(),
            self.stack_segment.clone(),
        )
    }
    pub fn push_code_vec(&mut self, code: Vec<Inst>) {
        self.code_segment.extend(code);
    }
    pub fn push_stack_vec(&mut self, stack: Vec<RegType>) {
        self.stack_segment.extend(stack);
    }
}
