/* mu/symbol.rs */
use crate::mu::r#type::Tag;
use crate::mu::r#type::Type;
use crate::mu::r#type::NIL;
use crate::mu::r#type::entag;
use crate::mu::r#type::detag;

use crate::mu::string::_string;

pub struct _Symbol {
    _name: Type,
    _value: Type,
}

pub struct _Keyword {
    _keyword: Type
}

pub fn _symbol(_name: Type, _value: Type) -> Type {
    let sym = _Symbol { _name, _value };
    
    Type::from_symbol(&sym)
}

pub fn _symbol_value(_symbol: Type) -> Type {
    let _sym = _symbol.symbol_from_type();

    NIL
}

impl _Symbol { }

impl Type {
    pub fn type_symbol(&self) -> bool {
        match self.tag() {
            Tag::Symbol => true,
            _ => false
        }
    }
    
    pub fn from_symbol(_sym: &_Symbol) -> Type {
        unsafe {
            let sym_addr: u64 = std::mem::transmute(_sym);
            entag(sym_addr << 3, Tag::Symbol)
        }        
    }
    
    pub fn symbol_from_type(self) -> &'static _Symbol {
        let sym: &_Symbol = unsafe { std::mem::transmute(detag(self)) };
        sym
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol() {
        assert!(_symbol(_string(&"whoa"), NIL).type_symbol());
    }
}
