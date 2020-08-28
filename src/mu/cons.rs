/* mu/cons.rs */
use crate::mu::r#type::Tag;
use crate::mu::r#type::Type;
use crate::mu::r#type::entag;

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

    pub fn cons(self, cdr: Type) -> Type {

        unsafe {
            let cons = &Cons {_car: self, _cdr: cdr };
            let cons_addr: u64 = std::mem::transmute(cons);
        
            entag(cons_addr, Tag::Cons)
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

/*
    #[test]
    fn test_type() {
        assert!(fixnum(0).type_());
    }

    #[test]
    fn test_u64() {
        assert!(fixnum(0).u64_of() == 0);
        assert!(fixnum(1).u64_of() == 1);
    }

    #[test]
    fn test_eq() {
        assert!(fixnum(0).eq(fixnum(0)));
    }
    
    #[test]
    fn test_add() {
        assert!(fixnum(1).add(fixnum(2)).eq(fixnum(3)));
    }
*/
}
