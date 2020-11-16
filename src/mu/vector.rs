/* mu/vector.rs */
use std::mem;

use crate::mu::env::Env;
use crate::mu::r#type::entag;
use crate::mu::r#type::{SysClass, Tag, Type};

#[derive(Debug)]
enum VecData {
    Char(&'static str),
    Byte(Vec<u8>),
    Fixnum(Vec<u64>),
    Float(Vec<f32>),
    T(&'static Vec<Type>),
}

#[derive(Debug)]
pub struct Vector {
    type_: SysClass,
    data_: VecData,
}

impl Vector {
    pub fn make_type(vec_: &'static Vec<Type>) -> Type {
        let v = Vector {
            type_: SysClass::T,
            data_: VecData::T(vec_),
        };
        unsafe {
            let addr: u64 = std::mem::transmute(&v);
            entag(addr << 3, Tag::Vector)
        }
    }

    pub fn vector_type(&self) -> &SysClass {
        &self.type_
    }

    pub fn evict(&self, env: &mut Env<'_>) -> Type {
        let vector = env.heap.alloc(mem::size_of::<Vector>(), Tag::Vector);
        unsafe {
            let dest: *mut u8 = std::mem::transmute(vector);
            let src: *const u8 = std::mem::transmute(&self);
            std::ptr::copy_nonoverlapping::<u8>(src, dest, mem::size_of::<Vector>());
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
