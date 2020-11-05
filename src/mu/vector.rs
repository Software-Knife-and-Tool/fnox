/* mu/vector.rs */
use std::mem;

use crate::mu::env::Env;
use crate::mu::r#type::entag;
use crate::mu::r#type::{SysClass, Tag, Type};

#[derive(Debug)]
enum Vector {
    _String(String),
}

#[derive(Debug)]
pub struct _Vector {
    _type: SysClass,
    _vector: Vector,
}

impl _Vector {
    pub fn _vector_type(&self) -> &SysClass {
        &self._type
    }

    pub fn evict(&self, env: &mut Env<'_>) -> Type {
        let vector = env.heap.alloc(mem::size_of::<_Vector>(), Tag::Vector);
        unsafe {
            let _dest: *mut u8 = std::mem::transmute(vector);
            let _src: *const u8 = std::mem::transmute(&self);
            std::ptr::copy_nonoverlapping::<u8>(_src, _dest, mem::size_of::<_Vector>());
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
