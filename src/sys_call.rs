use demo_isa::{err::ISAErr, ISARuner};

use crate::{cpu::CpuCore, memory::Memory};

use self::write::write_std;

mod write;
pub trait ISARunerExSysCall: ISARuner {
    fn run_sys_call(&mut self, mem: &mut Memory) -> Result<(), SysCallErr>;
}
#[derive(Debug)]
pub enum SysCallErr {
    InvalidSysCall,
    InvalidSysCallArg,
    WriteErr(write::WriteErr),
}
impl From<SysCallErr> for ISAErr {
    fn from(err: SysCallErr) -> ISAErr {
        match err {
            SysCallErr::InvalidSysCall => ISAErr::InvalidSysCall,
            SysCallErr::InvalidSysCallArg => ISAErr::InvalidSysCallArg,
            _ => ISAErr::SysCallErr,
        }
    }
}
type SysCall = fn(&mut CpuCore, &mut Memory) -> Result<(), SysCallErr>;
pub const SYS_CALL_TABLE: &[SysCall] = &[write_std];
