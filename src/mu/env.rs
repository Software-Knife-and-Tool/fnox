/* mu/env.rs */
use std::collections::HashMap;
use crate::mu::r#type::SysClass;
// use crate::mu::r#type::Tag;
use crate::mu::r#type::Type;
use crate::mu::r#type::NIL;
// use crate::mu::r#type::entag;
// use crate::mu::heap::heap;
// use crate::mu::heap::Heap;

use crate::mu::fixnum::_fixnum_add;

type FuncV = fn(_args: Vec<Type>) -> Type;

pub struct Env<'e> {
    // heap: Heap
    _symtab: HashMap<&'e str, FuncV>
}

// fn(args: Vec<Type>) -> Type
pub fn env<'e>() -> Env<'e> {
    let mut _init: HashMap<&'e str, FuncV> = HashMap::new();

    _init.insert("fixnum-add", _fixnum_add);

    Env {
        // heap: heap(1024 * 1024)
       _symtab: _init
    }
}

pub fn not_found(_args: Vec<Type>) -> Type {
    NIL
}

impl Env<'_> {
    pub fn eval(ptr: Type) -> Type {
        match ptr.type_of() {
            SysClass::Cons => ptr,
            SysClass::Fixnum => ptr,
            _ => ptr
        }
    }

    pub fn lookup(&self, name: &str) -> FuncV {
        match self._symtab.get(name) {
            Some(func) => *func,
            None => not_found
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symtab() {
        let env = env();
        assert!(env.lookup(&"fixnum-add") == _fixnum_add);
        assert!(env.lookup(&"nope") == not_found);
    }
}

