/* mu/stream.rs */
use std::mem;

use crate::mu::env::Env;
use crate::mu::r#type::{detag, entag, Tag, Type};

#[derive(Debug)]
pub struct _Stream {
    _name: Type,
    _func: fn(Vec<Type>) -> Type,
    _nargs: i16,
}

pub fn _stream(_name: Type, _func: fn(Vec<Type>) -> Type, _nargs: i16) -> Type {
    let fun = _Stream {
        _name,
        _func,
        _nargs,
    };

    Type::from_stream(&fun)
}

impl _Stream {
    pub fn evict(&self, env: &mut Env<'_>) -> Type {
        let stream = env.heap.alloc(mem::size_of::<_Stream>(), Tag::Cons);
        unsafe {
            let _dest: *mut u8 = std::mem::transmute(stream);
            let _src: *const u8 = std::mem::transmute(&self);
            std::ptr::copy_nonoverlapping::<u8>(_src, _dest, mem::size_of::<_Stream>());
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

    pub fn from_stream(_fn: &_Stream) -> Type {
        unsafe {
            let fn_addr: u64 = std::mem::transmute(_fn);
            entag(fn_addr << 3, Tag::Stream)
        }
    }

    pub fn stream_from_type(&self) -> &'static _Stream {
        let _fn: &_Stream = unsafe { std::mem::transmute(detag(self)) };
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
