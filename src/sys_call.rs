
use demo_isa::err::ISAErr;

use crate::{cpu::CpuCore, memory::Memory};

use self::write::write_std;

mod write;

#[derive(Debug)]
pub enum SysCallErr {
    InvalidSysCall,
    InvalidSysCallArg,
    WriteErr(write::WriteErr),
    ISAErr(ISAErr),
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
impl From<ISAErr> for SysCallErr {
    fn from(err: ISAErr) -> SysCallErr {
        SysCallErr::ISAErr(err)
    }
}
type SysCall = fn(&mut CpuCore, &mut Memory) -> Result<(), SysCallErr>;
pub const SYS_CALL_TABLE: &[SysCall] = &[write_std];
