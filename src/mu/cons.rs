/* mu/cons.rs */
use crate::mu::r#type::Tag;
use crate::mu::r#type::Type;
use crate::mu::r#type::NIL;
use crate::mu::r#type::entag;
use crate::mu::r#type::detag;

pub struct Cons {
    _car: Type,
    _cdr: Type
}

impl Type {
    pub fn type_cons(&self) -> bool {
        match self.tag() {
            Tag::Cons => true,
            _ => false
        }
    }

    pub fn type_list(&self) -> bool {
        self.eq(NIL) || self.type_cons()
    }

    pub fn from_cons(_cons: &Cons) -> Type {
        unsafe {
            let cons_addr: u64 = std::mem::transmute(_cons);
            entag(cons_addr, Tag::Cons)
        }        
    }
    
    pub fn cons(self, cdr: Type) -> Type {
        Type::from_cons(&Cons {_car: self, _cdr: cdr })
    }

    pub fn cons_from_type(self) -> &'static Cons {
        let cons: &Cons = unsafe { std::mem::transmute(detag(self)) };
        cons
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type() {
        assert!(NIL.cons(NIL).type_cons());
    }

    #[test]
    fn test_list() {
        assert!(NIL.type_list());
        assert!(NIL.cons(NIL).type_list());
    }

    /*
    #[test]
    fn test_cxr() {
        assert!(NIL.cons(NIL).cons_from_type()._car.eq(NIL));
    }

    #[test]
    fn test_cons() {
        let _cons = NIL.cons(NIL).type_cons();
        
        assert!(fixnum(0).u64_of() == 0);
        assert!(fixnum(1).u64_of() == 1);
    }
     */
}
