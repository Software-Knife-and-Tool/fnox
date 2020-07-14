/* mu/env.rs */

use crate::mu::r#type::Tag;
use crate::mu::r#type::Type;
use crate::mu::r#type::NIL;
use crate::mu::r#type::entag;

use crate::mu::heap::heap;
use crate::mu::heap::Heap;

pub struct Env {
    heap: Heap
}

pub fn env() -> Env {
    println!("making env, damnit");
    Env {
        heap: heap(1024 * 1024)
    }
}

impl Env {
    pub fn eval(ptr: Type) -> Type {
        ptr
    }
}

