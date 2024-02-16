use demo_isa::isa::RegType;

#[derive(Debug, Clone)]
pub enum HeapObj {
    R(RegType),
}

pub type HeapAddr = usize;
pub type Heap = Vec<HeapObj>;
