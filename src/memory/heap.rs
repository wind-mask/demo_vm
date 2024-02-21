use std::mem;

use demo_isa::{reg::UsizeRegType, RegType};

#[derive(Debug, Clone)]
pub enum HeapObj {
    R(RegType),
    UArray(Vec<UsizeRegType>),
    FArray(Vec<f64>),
}

impl HeapObj {
    // fn get_reg_type(&self) -> Result<RegType, demo_isa::err::ISAErr> {
    //     match self {
    //         HeapObj::R(r) => Ok(*r),
    //         HeapObj::UArray(u) => {
    //             if u.is_empty() {
    //                 return Err(demo_isa::err::ISAErr::InvalidHeapType);
    //             }
    //             Ok(RegType::Usize(u[0]))
    //         }
    //         Self::FArray(f) => {
    //             if f.is_empty() {
    //                 return Err(demo_isa::err::ISAErr::InvalidHeapType);
    //             }
    //             Ok(RegType::F64(f[0]))
    //         }
    //     }
    // }
    pub fn get_u8_vec(&self) -> &[u8] {
        match self {
            Self::R(RegType::Usize(u)) => unsafe {
                std::slice::from_raw_parts(
                    u as *const usize as *const u8,
                    mem::size_of::<UsizeRegType>(),
                )
            },
            Self::R(RegType::F64(f)) => unsafe {
                std::slice::from_raw_parts(f as *const f64 as *const u8, mem::size_of::<f64>())
            },
            Self::UArray(u) => {
                if u.is_empty() {
                    return &[0; mem::size_of::<UsizeRegType>()];
                }
                unsafe {
                    std::slice::from_raw_parts(
                        u.as_ptr() as *const u8,
                        u.len() * mem::size_of::<UsizeRegType>(),
                    )
                }
            }
            Self::FArray(f) => {
                if f.is_empty() {
                    return &[0; mem::size_of::<f64>()];
                }
                unsafe {
                    std::slice::from_raw_parts(
                        f.as_ptr() as *const u8,
                        f.len() * mem::size_of::<f64>(),
                    )
                }
            } // _ => Err(demo_isa::err::ISAErr::InvalidHeapType),
        }
    }
}
#[cfg(test)]
#[test]
fn test_get_u8_vec() {
    use rand::random;
    // 测试对于uszie的正确性
    for _ in 0..100 {
        let rand_u: usize = random();
        let heap_obj = HeapObj::R(RegType::Usize(rand_u));
        let u8_vec = heap_obj.get_u8_vec();
        assert_eq!(u8_vec.len(), mem::size_of::<UsizeRegType>());
        assert_eq!(*u8_vec, rand_u.to_ne_bytes());
    }
    // 测试对于f64的正确性
    for _ in 0..100 {
        let rand_f: f64 = random();
        let heap_obj = HeapObj::R(RegType::F64(rand_f));
        let u8_vec = heap_obj.get_u8_vec();
        assert_eq!(u8_vec.len(), mem::size_of::<f64>());
        assert_eq!(*u8_vec, rand_f.to_ne_bytes());
    }
    // 测试对于usize数组的正确性
    for _ in 0..100 {
        let mut rand_len: usize = random();
        rand_len = rand_len % 1000;
        rand_len += 1;
        let mut u_array = Vec::with_capacity(rand_len);
        for _ in 0..rand_len {
            u_array.push(random());
        }
        let heap_obj = HeapObj::UArray(u_array.clone());
        let u8_vec = heap_obj.get_u8_vec();
        assert_eq!(u8_vec.len(), rand_len * mem::size_of::<UsizeRegType>());
        for i in 0..rand_len {
            assert_eq!(
                u8_vec
                    [i * mem::size_of::<UsizeRegType>()..(i + 1) * mem::size_of::<UsizeRegType>()],
                u_array[i].to_ne_bytes()
            );
        }
    }
    // 测试对于f64数组的正确性
    for _ in 0..100 {
        let mut rand_len: usize = random();
        rand_len = rand_len % 1000;
        rand_len += 1;
        let mut f_array = Vec::with_capacity(rand_len);
        for _ in 0..rand_len {
            f_array.push(random());
        }
        let heap_obj = HeapObj::FArray(f_array.clone());
        let u8_vec = heap_obj.get_u8_vec();
        assert_eq!(u8_vec.len(), rand_len * mem::size_of::<f64>());
        for i in 0..rand_len {
            assert_eq!(
                u8_vec[i * mem::size_of::<f64>()..(i + 1) * mem::size_of::<f64>()],
                f_array[i].to_ne_bytes()
            );
        }
    }
}
impl HeapObj {
    pub fn get_reg_u_type(&self) -> Result<&demo_isa::reg::UsizeRegType, demo_isa::err::ISAErr> {
        match self {
            HeapObj::R(RegType::Usize(u)) => Ok(u),
            HeapObj::UArray(u) => {
                if u.is_empty() {
                    return Err(demo_isa::err::ISAErr::InvalidHeapType);
                }
                Ok(&u[0])
            }
            _ => Err(demo_isa::err::ISAErr::InvalidHeapType),
        }
    }
    pub fn get_reg_f_type(&self) -> Result<&demo_isa::reg::F64RegType, demo_isa::err::ISAErr> {
        match self {
            HeapObj::R(RegType::F64(f)) => Ok(f),
            HeapObj::FArray(f) => {
                if f.is_empty() {
                    return Err(demo_isa::err::ISAErr::InvalidHeapType);
                }
                Ok(&f[0])
            }
            _ => Err(demo_isa::err::ISAErr::InvalidHeapType),
        }
    }
}
