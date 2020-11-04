/* mu/exception.rs */
use crate::mu::r#type::{detag, entag, Tag, Type};

#[derive(Debug)]
pub struct _Exception {
    _name: Type,
    _func: fn(Vec<Type>) -> Type,
    _nargs: i16,
}

pub fn _exception(_name: Type, _func: fn(Vec<Type>) -> Type, _nargs: i16) -> Type {
    let fun = _Exception {
        _name,
        _func,
        _nargs,
    };

    Type::from_exception(&fun)
}

impl _Exception {}

impl Type {
    pub fn typep_exception(&self) -> bool {
        match self.tag() {
            Tag::Exception => true,
            _ => false,
        }
    }

    pub fn from_exception(_fn: &_Exception) -> Type {
        unsafe {
            let fn_addr: u64 = std::mem::transmute(_fn);
            entag(fn_addr << 3, Tag::Exception)
        }
    }

    pub fn exception_from_type(&self) -> &'static _Exception {
        let _ex: &_Exception = unsafe { std::mem::transmute(detag(self)) };
        _ex
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
