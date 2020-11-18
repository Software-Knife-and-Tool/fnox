// mu/vector.rs
use std::mem;

use crate::mu::env::FnEnv;
use crate::mu::r#type::entag;
use crate::mu::r#type::{SysClass, Tag, Type};

#[derive(Debug)]
enum VecData {
    _Char(&'static str),
    _Byte(Vec<u8>),
    _Fixnum(Vec<u64>),
    _Float(Vec<f32>),
    _T(&'static Vec<Type>),
}

#[derive(Debug)]
pub struct FnVector {
    type_: SysClass,
    data_: VecData,
}

impl FnVector {
    pub fn _make_type(vec_: &'static Vec<Type>) -> Type {
        let v = FnVector {
            type_: SysClass::T,
            data_: VecData::_T(vec_),
        };
        unsafe {
            let addr: u64 = std::mem::transmute(&v);
            entag(addr << 3, Tag::Vector)
        }
    }

    pub fn _vector_type(&self) -> &SysClass {
        &self.type_
    }

    pub fn _evict(&self, env: &mut FnEnv<'_>) -> Type {
        let vector = env.heap.alloc(mem::size_of::<FnVector>(), Tag::Vector);
        unsafe {
            let dest: *mut u8 = std::mem::transmute(vector);
            let src: *const u8 = std::mem::transmute(&self);
            std::ptr::copy_nonoverlapping::<u8>(src, dest, mem::size_of::<FnVector>());
        }
        assert!((vector & 0x7) == 0);
        entag(vector, Tag::Vector)
    }
}

impl Type {
    pub fn typep_vector(&self) -> bool {
        match self.tag() {
            Tag::Vector => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    /*
        use super::*;

        #[test]
        fn test_immed() {
            assert!(_T.eq(_T));
        }
    */
}
