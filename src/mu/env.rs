/* mu/env.rs */
use std::collections::HashMap;
use crate::mu::r#type::SysClass;
// use crate::mu::r#type::Tag;
use crate::mu::r#type::Type;
// use crate::mu::r#type::NIL;
// use crate::mu::r#type::entag;
// use crate::mu::heap::heap;
// use crate::mu::heap::Heap;

pub struct Env<'e> {
    // heap: Heap
    _symtab: HashMap<&'e str, i32>
}

pub fn env<'e>() -> Env<'e> {
    let _init: HashMap<&'e str, i32> =
        [("Norway", 100),
         ("Denmark", 50),
         ("Iceland", 10)]
        .iter().cloned().collect();
    
    Env {
        // heap: heap(1024 * 1024)
       _symtab: _init
    }
}

impl Env<'_> {
    pub fn eval(ptr: Type) -> Type {
        match ptr.type_of() {
            SysClass::Cons => ptr,
            SysClass::Fixnum => ptr,
            _ => ptr
        }
    }

    pub fn lookup(&self, name: &str) -> i32 {
        match self._symtab.get(name) {
            Some(number) => *number,
            None => -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symtab() {
        let env = env();
        assert!(env.lookup(&"Norway") == 100);
        assert!(env.lookup(&"nope") == -1);
    }
}

