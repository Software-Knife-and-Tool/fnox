/* mu/symbol.rs */
use crate::mu::r#type::Tag;
use crate::mu::r#type::Type;

use crate::mu::r#type::entag;
use crate::mu::r#type::detag;

use crate::mu::r#type::_immediate;
use crate::mu::r#type::ImmediateClass;

#[derive(Debug)]
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

pub fn _keyword(_name: Type) -> Option<Type> {

    if _name.type_string() {
        let str = _name._string_value();
        let len : u8 = str.len() as u8;
        let mut data : u64 = 0;

        for ch in str.chars() {
            data = (data << 8) + ch as u64;
        }
        
        let immed = _immediate(data, len, ImmediateClass::Keyword);

        Some(immed)
    } else {
        None
    }
}

impl _Symbol { }

impl Type {
    pub fn type_symbol(&self) -> bool {
        match self.tag() {
            Tag::Symbol => true,
            _ => false
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
    
    pub fn symbol_from_type(self) -> &'static _Symbol {
        let sym: &_Symbol = unsafe { std::mem::transmute(detag(self)) };
        sym
    }

    pub fn _symbol_name(self) -> Option<&'static Type> {
        if Type::type_symbol(&self) {
            let _sym = self.symbol_from_type();

            Some(&_sym._name)
        } else {
            None
        }
    }
    
    pub fn _symbol_value(self) -> Option<&'static Type> {
        if Type::type_symbol(&self) {
            let _sym = self.symbol_from_type();

            Some(&_sym._value)
        } else {
            None
        }
    }

}

#[cfg(test)]
mod tests {
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
}
