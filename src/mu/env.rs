/* mu/env.rs */
use std::collections::HashMap;
use crate::mu::r#type::SysClass;
// use crate::mu::r#type::Tag;
use crate::mu::r#type::Type;
// use crate::mu::r#type::NIL;
// use crate::mu::r#type::entag;
// use crate::mu::heap::heap;
// use crate::mu::heap::Heap;

pub struct Env<'e> {
    // heap: Heap
    _symtab: HashMap<&'e str, i32>
}

pub fn env<'e>() -> Env<'e> {
    let _init: HashMap<&'e str, i32> =
        [("Norway", 100),
         ("Denmark", 50),
         ("Iceland", 10)]
        .iter().cloned().collect();
    
    Env {
        // heap: heap(1024 * 1024)
       _symtab: _init
    }
}

impl Env<'_> {
    pub fn eval(ptr: Type) -> Type {
        match ptr.type_of() {
            SysClass::Cons => ptr,
            SysClass::Fixnum => ptr,
            _ => ptr
        }
    }
}

#[cfg(test)]
mod tests {
/*
    use super::*;

    #[test]
    fn test_type() {
        assert!(_fixnum(0).type_fixnum());
    }

    #[test]
    fn test_i64() {
        assert!(
            match _fixnum(0).i64_from_fixnum() {
                None => false,
                Some(v) => v == 0
            });

        assert!(
            match _fixnum(1).i64_from_fixnum() {
                None => false,
                Some(v) => v == 1
            });
    }

    #[test]
    fn test_eq() {
        assert!(_fixnum(0).eq(_fixnum(0)));
        assert!(!_fixnum(0).eq(_fixnum(1)));
    }

    #[test]
    fn test_add() {
        assert!(
            match _Fixnum::_from_type(&_fixnum(1)) {
                Some(fx) =>
                    match fx._add(&_fixnum(2)) {
                        Some(sum) =>
                            match sum.i64_from_fixnum() {
                                Some(v) => v == 3,
                                None => false
                            },
                        None => false
                    },
                None => false
            });
    }
*/
}

