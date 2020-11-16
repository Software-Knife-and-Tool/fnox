/* mu/function.rs */
use std::mem;

use crate::mu::r#type::NIL;
use crate::mu::r#type::{detag, entag, Tag, Type};

use crate::mu::env::Env;

#[derive(Debug)]
pub struct Function {
    name: Type,
    func: fn(Vec<Type>) -> Type,
    nargs: i16,
}

impl Function {
    pub fn make_type(name: Type, func: fn(Vec<Type>) -> Type, nargs: i16) -> Type {
        let fun = Function { name, func, nargs };

        Type::from_function(&fun)
    }

    pub fn evict(&self, env: &mut Env<'_>) -> Type {
        let fn_ = env.heap.alloc(mem::size_of::<Function>(), Tag::Cons);
        unsafe {
            let dest: *mut u8 = std::mem::transmute(fn_);
            let src: *const u8 = std::mem::transmute(&self);
            std::ptr::copy_nonoverlapping::<u8>(src, dest, mem::size_of::<Function>());
        }
        assert!((fn_ & 0x7) == 0);
        entag(fn_, Tag::Function)
    }
}

impl Type {
    pub fn funcall(&self, _env: &Env<'_>, _args: Type) -> Type {
        NIL
    }

    pub fn typep_function(&self) -> bool {
        match self.tag() {
            Tag::Function => true,
            _ => false,
        }
    }

    pub fn from_function(fn_: &Function) -> Type {
        unsafe {
            let addr: u64 = std::mem::transmute(fn_);
            entag(addr << 3, Tag::Function)
        }
    }

    pub fn function_from_type(&self) -> &'static Function {
        let fn_: &Function = unsafe { std::mem::transmute(detag(self)) };
        fn_
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
