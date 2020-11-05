/* mu/char.rs */
use std::char;

use crate::mu::r#type::{immediate, ImmediateClass};
use crate::mu::r#type::{Tag, Type};

#[derive(Debug)]
pub struct Char {
    char_: char,
}

pub fn char_(src: char) -> Type {
    immediate(src as u64, 0, ImmediateClass::Char)
}

impl Char {
    pub fn _print(&self) {
        println!("{}", self.char_);
    }

    pub fn _from_char(char_: char) -> Char {
        Char { char_: char_ }
    }

    pub fn _from_type(ch: &Type) -> Option<Char> {
        if Type::typep_char(ch) {
            Some(Char {
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
        assert!(char_('a').typep_char());
    }
}
