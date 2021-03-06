// mu/cons.rs
use std::mem;

use crate::mu::r#type::NIL;
use crate::mu::r#type::{detag, entag, Tag, Type};

use crate::mu::env::FnEnv;
use crate::mu::print::debug_println;

#[derive(Debug)]
pub struct FnCons {
    car: Type,
    cdr: Type,
}

impl FnCons {
    pub fn car(&self) -> Type {
        self.car
    }

    pub fn cdr(&self) -> Type {
        self.cdr
    }

    pub fn make_type(_car: Type, _cdr: Type) -> Type {
        NIL
    }

    pub fn evict(&self, env: &mut FnEnv<'_>) -> Type {
        let cons = env.heap.alloc(mem::size_of::<FnCons>(), Tag::Cons);
        unsafe {
            let _dest: *mut u8 = std::mem::transmute(cons);
            let _src: *const u8 = std::mem::transmute(&self);
            std::ptr::copy_nonoverlapping::<u8>(_src, _dest, mem::size_of::<FnCons>());
        }
        assert!((cons & 0x7) == 0);
        entag(cons, Tag::Cons)
    }
}

impl Type {
    pub fn typep_cons(&self) -> bool {
        match self.tag() {
            Tag::Cons => true,
            _ => false,
        }
    }

    pub fn typep_list(&self) -> bool {
        self.eq(NIL) || self.typep_cons()
    }

    pub fn from_cons(_cons: &FnCons) -> Type {
        unsafe {
            let cons_addr: u64 = std::mem::transmute(_cons);
            entag(cons_addr << 3, Tag::Cons)
        }
    }

    pub fn cons(&self, cdr: Type) -> Type {
        println!("(cons> ");
        debug_println(*self);
        debug_println(cdr);
        println!("cons<)");
        Type::from_cons(&FnCons {
            car: *self,
            cdr: cdr,
        })
    }

    pub fn cons_from_type(&self) -> &'static FnCons {
        let cons: &FnCons = unsafe { std::mem::transmute(detag(self)) };
        cons
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type() {
        assert!(NIL.cons(NIL).typep_cons());
    }

    #[test]
    fn test_list() {
        assert!(NIL.typep_list());
        assert!(NIL.cons(NIL).typep_list());
    }

    #[test]
    fn test_evict() {
        assert!(NIL.typep_list());
        assert!(NIL.cons(NIL).typep_list());
    }

    /*
    #[test]
    fn test_cxr() {
        assert!(NIL.cons(NIL).cons_from_type()._car.eq(NIL));
    }

    #[test]
    fn test_cons() {
        let _cons = NIL.cons(NIL).typep_cons();

        assert!(fixnum(0).u64_of() == 0);
        assert!(fixnum(1).u64_of() == 1);
    }
     */
}
