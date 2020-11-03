/* mu/symbol.rs */
use std::mem;

use crate::mu::r#type::{Type, Tag, entag, detag};
use crate::mu::r#type::{ImmediateClass, _immediate};

use crate::mu::env::{Env};

#[derive(Debug)]
pub struct _Symbol {
    pub _name: Type,
    pub _value: Type,
}

pub struct _Keyword {
    _keyword: Type
}

pub fn _symbol(_name: Type, _value: Type) -> Type {
    let sym = _Symbol { _name, _value };
    
    Type::from_symbol(&sym)
}

pub fn _keyword(name: Type) -> Type {

    assert!(name.is_immediate());
    let immed = _immediate(name.immediate_data(),
                           name.immediate_size(),
                           ImmediateClass::Keyword);
    immed
}

impl _Symbol {
    
    pub fn evict(&self, env: &mut Env<'_>) -> Type {
        let symbol = env.heap.alloc(mem::size_of::<_Symbol>(), Tag::Symbol);
        unsafe {
            let _dest: *mut u8 = std::mem::transmute(symbol);
            let _src: *const u8 = std::mem::transmute(&self);
            std::ptr::copy_nonoverlapping::<u8>(_src, _dest, mem::size_of::<_Symbol>());
        }
        assert!((symbol & 0x7) == 0);
        entag(symbol, Tag::Symbol)
    }

}

impl Type {

    pub fn type_symbol(&self) -> bool {
        match self.tag() {
            Tag::Symbol => true,
            _ => self.type_keyword()
        }
    }

    pub fn type_keyword(&self) -> bool {
        match self.tag() {
            Tag::Immediate =>
                match self.immediate_class() {
                    ImmediateClass::Keyword => true,
                    _ => false
                },
            _ => false
        }
    }

    pub fn from_symbol(_sym: &_Symbol) -> Type {
        unsafe {
            let sym_addr: u64 = std::mem::transmute(_sym);
            entag(sym_addr << 3, Tag::Symbol)
        }        
    }
    
    pub fn symbol_from_type(&self) -> &'static _Symbol {
        let sym: &_Symbol = unsafe { std::mem::transmute(detag(self)) };
        sym
    }

}

#[cfg(test)]
mod tests {
    /*
    use crate::mu::r#type::NIL;
    use crate::mu::string::_string;
    
    use super::*;

    #[test]
    fn test_symbol() {
        assert!(_symbol(_string(b"whoa"), NIL).type_symbol());
    }

    #[test]
    fn test_keyword() {
        assert!(
            match _keyword(_string(b"whoa")) {
                Some(kwd) => kwd.type_keyword(),
                None => false
            }
            );
    }
     */
}
