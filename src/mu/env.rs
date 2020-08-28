/* mu/env.rs */
use crate::mu::r#type::SysClass;
// use crate::mu::r#type::Tag;
use crate::mu::r#type::Type;
// use crate::mu::r#type::NIL;
// use crate::mu::r#type::entag;

// use crate::mu::heap::heap;
// use crate::mu::heap::Heap;

pub struct Env {
    // heap: Heap
}

pub fn env() -> Env {
    Env {
        // heap: heap(1024 * 1024)
    }
}

impl Env {
    pub fn eval(ptr: Type) -> Type {
        match ptr.type_of() {
            SysClass::Cons => ptr,
            SysClass::Symbol => ptr,
            _ => ptr
        }
    }
}

