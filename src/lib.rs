use crate::cpu::CpuCore;
use crate::memory::Memory;
use cpu::core::Regs;
use cpu::CpuErr;
use demo_isa::err::ISAErr;
use demo_isa::{Inst, RegType, VmRunner};
use memory::heap::HeapObj;
use memory::{Heap, Stack};

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
// #[cfg(test)]
pub mod cpu;
pub mod memory;
pub mod sys_call;
pub mod test;

#[derive(Debug)]
pub enum VmErr {
 CpuErr(CpuErr),   
 ISAErr(ISAErr),
}
impl From<ISAErr> for VmErr {
    fn from(err: ISAErr) -> VmErr {
        VmErr::ISAErr(err)
    }
}
impl From<CpuErr> for VmErr {
    fn from(err: CpuErr) -> VmErr {
        VmErr::CpuErr(err)
    }
    
}
#[derive(Debug)]
pub struct VmTmp {
    core: CpuCore,
    mem: Memory,
}

#[derive(Debug)]
pub struct Vm {
    regs:Regs,
    code_segment: Vec<Inst>,
    pub heap_segment: Heap,
    stack_segment: Vec<RegType>,
}
impl VmRunner for Vm{
    type VmErr = VmErr;
    fn run(&mut self,code: &[Inst])->Result<(), VmErr> {
        for inst in code {
            Vm::run_inst(self,inst)?;
        }
        Ok(())
    }
}
impl Vm{
    fn run_inst(vm:&mut Vm,inst:&Inst) -> Result<(), ISAErr> {
        match *inst {//TODO:implement all instructions
            Inst::Nop => {},
            Inst::MU(dur, uv) => {
                vm.regs.set_u_reg(dur, uv);
            }
            Inst::MD(_, _) => todo!(),
            Inst::MovU(_, _) => todo!(),
            Inst::MovD(_, _) => todo!(),
            Inst::Mod(_, _, _) => todo!(),
            Inst::AddU(_, _, _) => todo!(),
            Inst::AddUI(_, _) => todo!(),
            Inst::AddD(_, _, _) => todo!(),
            Inst::AddDI(_, _) => todo!(),
            Inst::SubU(_, _, _) => todo!(),
            Inst::SubUI(_, _) => todo!(),
            Inst::SubD(_, _, _) => todo!(),
            Inst::SubDI(_, _) => todo!(),
            Inst::MulU(_, _, _) => todo!(),
            Inst::MulD(_, _, _) => todo!(),
            Inst::DivU(_, _, _) => todo!(),
            Inst::DivD(_, _, _) => todo!(),
            Inst::And(_, _, _) => todo!(),
            Inst::Or(_, _, _) => todo!(),
            Inst::Xor(_, _, _) => todo!(),
            Inst::Not(_, _) => todo!(),
            Inst::NegU(_, _) => todo!(),
            Inst::NegD(_, _) => todo!(),
            Inst::Shl(_, _) => todo!(),
            Inst::Shr(_, _) => todo!(),
            Inst::LoadUH(_, _) => todo!(),
            Inst::LoadDH(_, _) => todo!(),
            Inst::LoadUS(_, _) => todo!(),
            Inst::LoadDS(_, _) => todo!(),
            Inst::StoreUH(_, _) => todo!(),
            Inst::StoreDH(_, _) => todo!(),
            Inst::StoreUS(_, _) => todo!(),
            Inst::StoreDS(_, _) => todo!(),
            Inst::Jo(_) => todo!(),
            Inst::Jno(_) => todo!(),
            Inst::Je(_, _, _) => todo!(),
            Inst::Jne(_, _, _) => todo!(),
            Inst::Jz(_, _) => todo!(),
            Inst::Jnz(_, _) => todo!(),
            Inst::Jmp(_) => todo!(),
            Inst::PushU(_) => todo!(),
            Inst::PushD(_) => todo!(),
            Inst::PopU(_) => todo!(),
            Inst::PopD(_) => todo!(),
            Inst::Call(_) => todo!(),
            Inst::SysCall(_) => todo!(),
            Inst::InU(_, _) => todo!(),
            Inst::InD(_, _) => todo!(),
            Inst::OutU(_, _) => todo!(),
            Inst::OutD(_, _) => todo!(),
            Inst::Ret => todo!(),
            Inst::Halt => todo!(),
        }
        Ok(())
    }
    
}

impl Default for VmTmp {
    fn default() -> Self {
        Self::new()
    }
}

impl VmTmp {
    pub fn new() -> VmTmp {
        VmTmp {
            core: CpuCore::new(),
            mem: Memory::new(),
        }
    }
    pub fn start(&mut self) -> Result<(), VmErr> {
        Ok(self.core.start(&mut self.mem)?)
    }
    pub fn set_code(&mut self, code: Vec<Inst>) {
        self.mem.store(Some(code), None, None);
    }
    pub fn mem_store(&mut self, code: Option<Vec<Inst>>, heap: Option<Heap>, stack: Option<Stack>) {
        self.mem.store(code, heap, stack);
    }
    pub fn mem_load(&self) -> (Vec<Inst>, Vec<HeapObj>, Vec<RegType>) {
        self.mem.load()
    }
    pub fn reset(&mut self) {
        self.core.reset();
        self.mem.reset();
    }
}
