/* mu/string.rs */
use crate::mu::r#type::NIL;
use crate::mu::r#type::{detag, entag, Tag, Type};
use crate::mu::r#type::{immediate, ImmediateClass, IMMEDIATE_STR_MAX};

#[derive(Debug)]
pub struct _String {
    pub _value: Type,
}

pub fn string(chars: &[u8]) -> Type {
    match std::str::from_utf8(chars) {
        Ok(str) => Type::from_string(str),
        Err(_) => NIL,
    }
}

impl Type {
    pub fn typep_string(&self) -> bool {
        match self.tag() {
            Tag::Vector => true,
            _ => false,
        }
    }

    pub fn from_string(str: &str) -> Type {
        if str.len() <= IMMEDIATE_STR_MAX {
            let mut chars : u64 = 0;
            for ch in str.as_bytes() {
                chars = (chars << 8) | *ch as u64;
            }
            immediate(chars, str.len() as u8, ImmediateClass::String)
        } else {
            unsafe {
                let addr: u64 = std::mem::transmute(&str);
                entag(addr << 3, Tag::Vector)
            }
        }
    }

    pub fn str_from_type(&self) -> &'static &str {
        match self.tag() {
            Tag::Immediate => b"",
            Tag::Vector => {
                let str: &_String = unsafe { std::mem::transmute(detag(self)) };
                &str
            }
        }
    }
}

#[cfg(test)]
mod tests {
    /*
    use super::*;

    #[test]
    fn test_string() {
        assert!(_string(b"yep").typep_string());
        assert!(
            match Type::str_from_type(_string(b"astring")) {
                Some(_) => true,
                None => false
        });
    }
    */
}
