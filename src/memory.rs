pub mod heap;
pub mod stack;

use demo_isa::err::{CpuErr, ISAErr};
use demo_isa::{HeapObjRuner, Inst, MemoryRuner, RegType, StackAddr};

#[derive(Debug)]
pub struct Memory {
    code_segment: Vec<Inst>,
    heap_segment: Heap,
    stack_segment: Vec<RegType>,
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
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
}

impl MemoryRuner for Memory {
    fn clear_heap(&mut self) {
        self.heap_segment.clear();
    }

    fn clear_stack(&mut self) {
        self.stack_segment.clear();
    }

    fn clear_code(&mut self) {
        self.code_segment.clear();
    }

    fn get_heap(&mut self, addr: usize) -> RegType {
        if let Some(obj) = self.heap_segment.get(addr) {
            obj.get_reg_type()
        } else {
            self.heap_segment
                .resize(addr + 1, HeapObj::R(RegType::Usize(0)));
            RegType::Usize(0)
        }
    }

    fn set_heap(&mut self, addr: usize, val: RegType) {
        if let Some(obj) = self.heap_segment.get_mut(addr) {
            obj.set_reg_type(val);
        } else {
            self.heap_segment
                .resize(addr + 1, HeapObj::R(RegType::Usize(0)));
            self.heap_segment[addr] = HeapObj::R(val);
        }
    }

    fn get_stack(&self, bp: StackAddr, addr: StackAddr) -> Result<RegType, ISAErr> {
        if let Some(val) = self.stack_segment.get(bp + addr) {
            Ok(*val)
        } else {
            Err(ISAErr::InvalidStackAddr)
        }
    }

    fn set_stack(&mut self, bp: StackAddr, addr: StackAddr, val: RegType) -> Result<(), ISAErr> {
        if let Some(obj) = self.stack_segment.get_mut(bp + addr) {
            *obj = val;
            Ok(())
        } else {
            Err(ISAErr::InvalidStackAddr)
        }
    }

    fn push_stack(&mut self, val: RegType) {
        self.stack_segment.push(val);
    }

    fn pop_stack(&mut self) -> Result<RegType, ISAErr> {
        if let Some(val) = self.stack_segment.pop() {
            Ok(val)
        } else {
            Err(ISAErr::InvalidStackAddr)
        }
    }

    fn get_stack_top_addr(&self) -> StackAddr {
        if self.stack_segment.is_empty() {
            0
        } else {
            self.stack_segment.len() - 1
        }
    }
    fn drop_stack_bp(&mut self, bp: StackAddr) {
        self.stack_segment.truncate(bp + 1);
    }
    fn fetch_code(&self, addr: demo_isa::CodeAddr) -> Result<Inst, CpuErr> {
        if addr < self.code_segment.len() {
            Ok(self.code_segment[addr])
        } else {
            Err(CpuErr::InvalidCodeAddr)
        }
    }
    fn push_code_vec(&mut self, code: Vec<Inst>) {
        self.code_segment.extend(code);
    }
    fn push_stack_vec(&mut self, stack: Vec<RegType>) {
        self.stack_segment.extend(stack);
    }
}

#[derive(Debug, Clone)]
pub enum HeapObj {
    R(RegType),
}

// pub type HeapAddr = usize;
pub type Heap = Vec<HeapObj>;

pub type Stack = Vec<RegType>;
