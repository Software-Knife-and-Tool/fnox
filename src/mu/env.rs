/* mu/env.rs */
use std::collections::HashMap;
use crate::mu::r#type::SysClass;
// use crate::mu::r#type::Tag;
use crate::mu::r#type::Type;
use crate::mu::r#type::NIL;
use crate::mu::read::_read;
use crate::mu::print::_print;
// use crate::mu::r#type::entag;
// use crate::mu::heap::Heap;

// use crate::mu::fixnum::_fixnum_add;
// use crate::mu::function::_function;
// use crate::mu::string::_string;

pub struct Env<'e> {
    // heap: Heap
    symtab: HashMap<&'e str, Type>
}

pub fn env<'e>() -> Env<'e> {
    let init: HashMap<&'e str, Type> = HashMap::new();

//    init.insert("fixnum-add",
//                 _function(_string(b"fixnum-add"),
//                           _fixnum_add,
//                           2));

    Env {
        // heap: heap(1024 * 1024)
        symtab: init
    }
}

impl Env<'_> {
    pub fn read(&self) -> Type {
        _read()
    }
    pub fn print(&self, src: Type) {
        _print(src);
    }

    pub fn eval(ptr: &'static Type) -> &'static Type {
        match ptr.type_of() {
            SysClass::Cons => ptr,
            SysClass::Symbol => ptr,
            /*
                match ptr._symbol_value() {
                    Some(v) => v,
                    None => &NIL
                },
             */
            SysClass::Fixnum => ptr,
            _ => ptr
        }
    }

    pub fn lookup(&self, name: &str) -> &Type {
        match self.symtab.get(name) {
            Some(_type) => _type,
            None => &NIL
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
