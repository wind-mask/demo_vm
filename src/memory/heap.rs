use crate::memory::HeapObj;
use demo_isa::{HeapObjRuner, RegType};

impl HeapObjRuner for HeapObj {
    fn get_reg_type(&self) -> RegType {
        match self {
            HeapObj::R(r) => *r,
        }
    }
    fn set_reg_type(&mut self, val: RegType) {
        match self {
            HeapObj::R(r) => *r = val,
        }
    }
}
