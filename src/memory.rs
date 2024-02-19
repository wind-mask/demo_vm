pub mod heap;
pub mod stack;

use demo_isa::err::{CpuErr, ISAErr};
use demo_isa::{Inst, MemoryRuner, RegType, StackAddr};

use self::heap::HeapObj;

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
    pub fn reset(&mut self) {
        self.code_segment.clear();
        self.heap_segment.clear();
        self.stack_segment.clear();
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
        if let Some(inst) = self.code_segment.get(addr) {
            Ok(*inst)
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

    fn get_heap_u_type(
        &mut self,
        addr: demo_isa::reg::UsizeRegType,
    ) -> Result<demo_isa::reg::UsizeRegType, ISAErr> {
        if let Some(h) = self.heap_segment.get(addr) {
            h.get_reg_u_type()
        } else {
            self.heap_segment
                .resize(addr + 1, HeapObj::R(RegType::Usize(0)));
            Ok(0)
        }
    }

    fn get_heap_f_type(
        &mut self,
        addr: demo_isa::reg::UsizeRegType,
    ) -> Result<demo_isa::reg::F64RegType, ISAErr> {
        if let Some(h) = self.heap_segment.get(addr) {
            h.get_reg_f_type()
        } else {
            self.heap_segment
                .resize(addr + 1, HeapObj::R(RegType::F64(0.0)));
            Ok(0.0)
        }
    }

    fn set_heap(&mut self, addr: demo_isa::reg::UsizeRegType, val: RegType) {
        if let Some(h) = self.heap_segment.get_mut(addr) {
            *h = HeapObj::R(val);
        } else {
            self.heap_segment
                .resize(addr + 1, HeapObj::R(RegType::Usize(0)));
            self.heap_segment[addr] = HeapObj::R(val);
        }
    }
}

impl Memory {
    pub fn get_heap_obj(&mut self, addr: demo_isa::reg::UsizeRegType) -> &HeapObj {
        if addr >= self.heap_segment.len() {
            self.heap_segment
                .resize(addr + 1, HeapObj::R(RegType::Usize(0)));
        }
        &self.heap_segment[addr]
    }
}

// pub type HeapAddr = usize;
pub type Heap = Vec<HeapObj>;

pub type Stack = Vec<RegType>;
