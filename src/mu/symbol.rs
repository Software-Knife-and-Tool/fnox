/* mu/symbol.rs */
use std::mem;

use crate::mu::r#type::NIL;
use crate::mu::r#type::{detag, entag, Tag, Type};
use crate::mu::r#type::{immediate, ImmediateClass};

use crate::mu::env::Env;

#[derive(Debug)]
pub struct Symbol {
    pub name: Type,
    pub value: Type,
}

#[derive(Debug)]
pub struct Keyword {
    keyword: Type,
}

pub fn symbol(name: Type, value: Type) -> Type {
    let sym = Symbol { name, value };

    Type::from_symbol(&sym)
}

pub fn keyword(name: Type) -> Type {
    match name.tag() {
        Tag::Immediate => immediate(
            name.immediate_data(),
            name.immediate_size(),
            ImmediateClass::Keyword,
        ),
        _ => NIL,
    }
}

impl Symbol {
    pub fn evict(&self, env: &mut Env<'_>) -> Type {
        let symbol = env.heap.alloc(mem::size_of::<Symbol>(), Tag::Symbol);
        unsafe {
            let dest: *mut u8 = std::mem::transmute(symbol);
            let src: *const u8 = std::mem::transmute(&self);
            std::ptr::copy_nonoverlapping::<u8>(src, dest, mem::size_of::<Symbol>());
        }
        assert!((symbol & 0x7) == 0);
        entag(symbol, Tag::Symbol)
    }
}

impl Type {
    pub fn typep_keyword(&self) -> bool {
        match self.tag() {
            Tag::Immediate => match self.immediate_class() {
                ImmediateClass::Keyword => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn typep_symbol(&self) -> bool {
        match self.tag() {
            Tag::Symbol => true,
            _ => self.typep_keyword(),
        }
    }

    pub fn from_symbol(sym: &Symbol) -> Type {
        unsafe {
            let addr: u64 = std::mem::transmute(sym);
            entag(addr << 3, Tag::Symbol)
        }
    }

    pub fn symbol_from_type(&self) -> &'static Symbol {
        let sym: &Symbol = unsafe { std::mem::transmute(detag(self)) };
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
        assert!(_symbol(_string(b"whoa"), NIL).typep_symbol());
    }

    #[test]
    fn test_keyword() {
        assert!(
            match _keyword(_string(b"whoa")) {
                Some(kwd) => kwd.typep_keyword(),
                None => false
            }
            );
    }
     */
}
