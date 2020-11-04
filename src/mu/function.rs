/* mu/function.rs */
use crate::mu::r#type::NIL;
use crate::mu::r#type::{detag, entag, Tag, Type};

#[derive(Debug)]
pub struct _Function {
    _name: Type,
    _func: fn(Vec<Type>) -> Type,
    _nargs: i16,
}

pub fn _function(_name: Type, _func: fn(Vec<Type>) -> Type, _nargs: i16) -> Type {
    let fun = _Function {
        _name,
        _func,
        _nargs,
    };

    Type::from_function(&fun)
}

impl _Function {
    pub fn funcall(&self, _args: Vec<Type>) -> Type {
        NIL
    }
}

impl Type {
    pub fn typep_function(&self) -> bool {
        match self.tag() {
            Tag::Function => true,
            _ => false,
        }
    }

    pub fn from_function(_fn: &_Function) -> Type {
        unsafe {
            let fn_addr: u64 = std::mem::transmute(_fn);
            entag(fn_addr << 3, Tag::Function)
        }
    }

    pub fn function_from_type(&self) -> &'static _Function {
        let _fn: &_Function = unsafe { std::mem::transmute(detag(self)) };
        _fn
    }
}

#[cfg(test)]
mod tests {
    /*
    use super::*;

    #[test]
    fn test_type() {
        assert!(NIL.cons(NIL).typep_function());
    }
     */
}
