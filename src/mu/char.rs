/* mu/char.rs */
use std::char;
// use std::mem;

use crate::mu::r#type::{ImmediateClass, _immediate};
use crate::mu::r#type::{Tag, Type};

#[derive(Debug)]
pub struct _Char {
    _char: char,
}

pub fn _char(src: char) -> Type {
    _immediate(src as u64, 0, ImmediateClass::Char)
}

impl _Char {
    pub fn _print(&self) {
        println!("{}", self._char);
    }

    pub fn _from_char(_char: char) -> _Char {
        _Char { _char: _char }
    }

    pub fn _from_type(ch: &Type) -> Option<_Char> {
        if Type::typep_char(ch) {
            Some(_Char { _char: char::from_u32(ch.immediate_data() as u32).unwrap() })
        } else {
            None
        }
    }
}

impl Type {
    pub fn typep_char(&self) -> bool {
        match self.tag() {
            Tag::Immediate => match Type::immediate_class(self) {
                ImmediateClass::Char => true,
                _ => false,
            },
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type() {
        assert!(_char('a').typep_char());
    }
}
