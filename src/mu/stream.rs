/* mu/stream.rs */
use crate::mu::r#type::{Type, Tag, entag, detag};

#[derive(Debug)]
pub struct _Stream {
    _name: Type,
    _func: fn(Vec<Type>) -> Type,
    _nargs: i16
}

pub fn _stream(_name: Type, _func: fn(Vec<Type>) -> Type, _nargs: i16) -> Type {
    let fun = _Stream { _name, _func, _nargs };
    
    Type::from_stream(&fun)
}

impl _Stream {

}

impl Type {
    pub fn type_stream(&self) -> bool {
        match self.tag() {
            Tag::Stream => true,
            _ => false
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
        assert!(NIL.cons(NIL).type_function());
    }
     */
}
