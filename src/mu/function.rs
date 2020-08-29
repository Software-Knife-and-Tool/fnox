/* mu/function.rs */
use crate::mu::r#type::Tag;
use crate::mu::r#type::Type;
// use crate::mu::r#type::NIL;
// use crate::mu::r#type::entag;
// use crate::mu::r#type::detag;

pub struct _Function {
    name: Type,
    func: fn(Vec<Type>) -> Type,
    nargs: i32
}

impl Type {
    pub fn type_function(&self) -> bool {
        match self.tag() {
            Tag::Function => true,
            _ => false
        }
    }
}

#[cfg(test)]
mod tests {
    /*
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
