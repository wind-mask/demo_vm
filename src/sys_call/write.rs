use std::io::{self, Error, Write};
use std::str::Utf8Error;

use demo_isa::{ISARuner, MemoryRuner, RegType};

use crate::cpu::CpuCore;
use crate::memory::Memory;

use super::SysCallErr;
use demo_isa::reg::Reg;
#[derive(Debug)]
pub enum WriteErr {
    UTF8Err(std::str::Utf8Error),
    IOError(Error),
}
impl From<Utf8Error> for WriteErr {
    fn from(err: Utf8Error) -> Self {
        WriteErr::UTF8Err(err)
    }
}
impl From<Error> for WriteErr {
    fn from(err: Error) -> Self {
        WriteErr::IOError(err)
    }
}
impl From<WriteErr> for SysCallErr {
    fn from(err: WriteErr) -> Self {
        SysCallErr::WriteErr(err)
    }
}
/// 写入标准输出
///
/// 参数：
///     R3: 写入的字符串的地址
///     R4: 写入的字符串的长度
///
/// 返回值：
///     R11: 0表示成功，其他表示失败
///     R12: 写入的字符数
#[allow(unused)]
pub fn write_std(core: &mut CpuCore, mem: &mut Memory) -> Result<(), SysCallErr> {
    let addr = match core.get_reg(Reg::R3) {
        RegType::Usize(addr) => addr,
        _ => return Err(SysCallErr::InvalidSysCallArg),
    };
    let len = match core.get_reg(Reg::R4) {
        RegType::Usize(len) => len,
        _ => return Err(SysCallErr::InvalidSysCallArg),
    };
    let mut buf = Vec::with_capacity(len);
    for i in 0..len {
        let c = match mem.get_heap(addr + i) {
            RegType::Usize(c) => c.to_ne_bytes(),
            _ => return Err(SysCallErr::InvalidSysCallArg),
        };
        buf.extend_from_slice(&c);
    }
    match io::stdout().write(&buf) {
        Ok(l) => {
            core.set_reg(Reg::R11, RegType::Usize(0));
            core.set_reg(Reg::R12, RegType::Usize(l));
            Ok(())
        }
        Err(e) => {
            core.set_reg(Reg::R11, RegType::Usize(1));
            core.set_reg(Reg::R12, RegType::Usize(0));
            Err(WriteErr::IOError(e).into())
        }
    }
}
