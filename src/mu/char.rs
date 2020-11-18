/* mu/char.rs */
use std::char;

use crate::mu::r#type::{immediate, ImmediateClass};
use crate::mu::r#type::{Tag, Type};

#[derive(Debug)]
pub struct FnChar {
    char_: char,
}

impl FnChar {
    pub fn make_type(src: char) -> Type {
        immediate(src as u64, 0, ImmediateClass::Char)
    }

    pub fn _print(&self) {
        println!("{}", self.char_);
    }

    pub fn _from_char(char_: char) -> FnChar {
        FnChar { char_: char_ }
    }

    pub fn _from_type(ch: &Type) -> Option<FnChar> {
        if Type::typep_char(ch) {
            Some(FnChar {
                char_: char::from_u32(ch.immediate_data() as u32).unwrap(),
            })
        } else {
            assert!(false);
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
        assert!(FnChar::make_type('a').typep_char());
    }
}
