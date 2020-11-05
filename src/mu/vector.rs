/* mu/vector.rs */
use std::mem;

use crate::mu::env::Env;
use crate::mu::r#type::entag;
use crate::mu::r#type::{SysClass, Tag, Type};

#[derive(Debug)]
enum VectorTypes {
    String(String),
}

#[derive(Debug)]
pub struct Vector {
    type_: SysClass,
}

impl Vector {
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
    pub fn is_vector(&self) -> bool {
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
