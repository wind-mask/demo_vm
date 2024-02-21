use std::io::{self, Error, Write};
use std::str::Utf8Error;

use crate::cpu::CpuCore;
use crate::memory::Memory;

use super::SysCallErr;

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
///     U2: 写入的字符串的地址
///     U3: 写入的字符串的长度
///
/// 返回值：
///     U4: 0表示成功，其他表示失败
///     U5: 写入的字符数
pub fn write_std(core: &mut CpuCore, mem: &mut Memory) -> Result<(), SysCallErr> {
    let addr = core.get_u_reg(demo_isa::reg::UsizeReg::U2);
    let len = core.get_u_reg(demo_isa::reg::UsizeReg::U3);
    let mut buf = Vec::with_capacity(len);
    for _ in 0..len {
        let c = mem.get_heap_obj(addr).get_u8_vec();
        buf.extend_from_slice(c);
    }
    match io::stdout().write(&buf) {
        Ok(l) => {
            core.set_u_reg(demo_isa::reg::UsizeReg::U4, 0);
            core.set_u_reg(demo_isa::reg::UsizeReg::U5, l);
            Ok(())
        }
        Err(e) => {
            core.set_u_reg(demo_isa::reg::UsizeReg::U4, 1);
            Err(WriteErr::IOError(e).into())
        }
    }
}
