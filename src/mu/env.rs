/* mu/env.rs */
use std::collections::HashMap;

use crate::mu::heap::{Heap, _heap};

use crate::mu::r#type::NIL;
use crate::mu::r#type::{SysClass, Tag, Type};

use crate::mu::print::_print;
use crate::mu::read::_read;

use crate::mu::fixnum::fixnum_add;
use crate::mu::function::Function;
use crate::mu::string::String;

pub struct Env<'e> {
    pub heap: Heap,
    pub symtab: HashMap<&'e str, Type>,
}

pub fn env<'e>() -> Env<'e> {
    let mut init: HashMap<&'e str, Type> = HashMap::new();

    init.insert(
        "fixnum-add",
        Function::make_type(String::make_type("fixnum-add"), fixnum_add, 2),
    );
    Env {
        heap: _heap(1024 * 1024),
        symtab: init,
    }
}

impl Env<'_> {
    pub fn read(&self) -> Type {
        _read()
    }
    pub fn print(&self, src: Type) {
        _print(src);
    }

    pub fn eval(&self, ptr: Type) -> Type {
        match ptr.tag() {
            Tag::Cons => {
                let cons = ptr.cons_from_type();
                let fn_ = cons.car();

                fn_.funcall(self, NIL)
            }
            Tag::Fixnum => ptr,
            Tag::Exception => ptr,
            Tag::Function => ptr,
            Tag::Stream => ptr,
            Tag::Symbol => {
                let sym = Type::symbol_from_type(&ptr);
                *sym.value()
            }
            Tag::Vector => ptr,
            Tag::Immediate => ptr,
        }
    }

    pub fn lookup(&self, name: &str) -> Type {
        match self.symtab.get(name) {
            Some(_type) => *_type,
            None => NIL,
        }
    }
}

#[cfg(test)]
mod tests {
    /*
        use super::*;

        #[test]
        fn test_symtab() {
            let env = env();
            assert!(!env.lookup(&"fixnum-add").eq(NIL));
            assert!(env.lookup(&"nope").eq(NIL));
        }
    */
}
