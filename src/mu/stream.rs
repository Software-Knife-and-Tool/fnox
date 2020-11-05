/* mu/stream.rs */
use std::mem;

use crate::mu::env::Env;
use crate::mu::r#type::{detag, entag, Tag, Type};

#[derive(Debug)]
pub struct Stream {
    name: Type,
    func: fn(Vec<Type>) -> Type,
    nargs: i16,
}

pub fn _stream(name: Type, func: fn(Vec<Type>) -> Type, nargs: i16) -> Type {
    let fun = Stream { name, func, nargs };

    Type::from_stream(&fun)
}

impl Stream {
    pub fn evict(&self, env: &mut Env<'_>) -> Type {
        let stream = env.heap.alloc(mem::size_of::<Stream>(), Tag::Cons);
        unsafe {
            let dest: *mut u8 = std::mem::transmute(stream);
            let src: *const u8 = std::mem::transmute(&self);
            std::ptr::copy_nonoverlapping::<u8>(src, dest, mem::size_of::<Stream>());
        }
        assert!((stream & 0x7) == 0);
        entag(stream, Tag::Stream)
    }
}

impl Type {
    pub fn typep_stream(&self) -> bool {
        match self.tag() {
            Tag::Stream => true,
            _ => false,
        }
    }

    pub fn from_stream(stream: &Stream) -> Type {
        unsafe {
            let addr: u64 = std::mem::transmute(stream);
            entag(addr << 3, Tag::Stream)
        }
    }

    pub fn stream_from_type(&self) -> &'static Stream {
        let stream: &Stream = unsafe { std::mem::transmute(detag(self)) };
        stream
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
