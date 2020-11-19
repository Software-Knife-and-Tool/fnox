// mu/exception.rs
use crate::mu::print::{debug_println, to_string};
use crate::mu::r#type::{detag, entag, Tag, Type};

#[derive(Debug)]
pub struct FnException {
    name: Type,
    func: fn(Vec<Type>) -> Type,
    nargs: i16,
}

pub fn error(src: Type, reason: &str) {
    println!("error: {} {}", to_string(src), reason);
    debug_println(src);

    assert!(false);
}

pub fn _exception(name: Type, func: fn(Vec<Type>) -> Type, nargs: i16) -> Type {
    let fun = FnException { name, func, nargs };

    Type::from_exception(&fun)
}

impl FnException {}

impl Type {
    pub fn typep_exception(&self) -> bool {
        match self.tag() {
            Tag::Exception => true,
            _ => false,
        }
    }

    pub fn from_exception(fn_: &FnException) -> Type {
        unsafe {
            let fn_addr: u64 = std::mem::transmute(fn_);
            entag(fn_addr << 3, Tag::Exception)
        }
    }

    pub fn exception_from_type(&self) -> &'static FnException {
        let ex: &FnException = unsafe { std::mem::transmute(detag(self)) };
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
