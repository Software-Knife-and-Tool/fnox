/* mu/exception.rs */
use crate::mu::r#type::{detag, entag, Tag, Type};

#[derive(Debug)]
pub struct Exception {
    name: Type,
    func: fn(Vec<Type>) -> Type,
    nargs: i16,
}

pub fn exception(name: Type, func: fn(Vec<Type>) -> Type, nargs: i16) -> Type {
    let fun = Exception { name, func, nargs };

    Type::from_exception(&fun)
}

impl Exception {}

impl Type {
    pub fn typep_exception(&self) -> bool {
        match self.tag() {
            Tag::Exception => true,
            _ => false,
        }
    }

    pub fn from_exception(fn_: &Exception) -> Type {
        unsafe {
            let fn_addr: u64 = std::mem::transmute(fn_);
            entag(fn_addr << 3, Tag::Exception)
        }
    }

    pub fn exception_from_type(&self) -> &'static Exception {
        let ex: &Exception = unsafe { std::mem::transmute(detag(self)) };
        ex
    }
}

#[cfg(test)]
mod tests {
    /*
    use super::*;

    #[test]
    fn test_type() {
        assert!(NIL.cons(NIL).typep_exception());
    }
     */
}
